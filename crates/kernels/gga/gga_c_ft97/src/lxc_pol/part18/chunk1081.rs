//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1081/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1081<F: Float>(t23084: F, t487: F, t1882: F, t23350: F, t23128: F, t1637: F, t5665: F, t5667: F, t1317: F, t5680: F, t23054: F, t23059: F, t38456: F, t91: F, t1318: F, t7943: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92088 = t487 * t23084;
    let t92096 = t1882 * t23350;
    let t92103 = t23128 * t487;
    let t92140 = t5665 * t1637 * t5667;
    let t92143 = t1317 * t1637 * t5680;
    let t92161 = t23054 * t23059;
    let t92162 = t92161 / 9.0;
    let t92173 = t91 * t38456;
    let t92185 = t1317 * t7943 * t1318;
    (t92088, t92096, t92103, t92140, t92143, t92161, t92162, t92173, t92185)
}
