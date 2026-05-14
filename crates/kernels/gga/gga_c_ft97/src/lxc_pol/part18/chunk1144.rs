//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1144/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1144<F: Float>(t92103: F, t979: F, t1643: F, t25899: F, t5674: F, t7793: F, t1307: F, t38477: F, t11064: F, t1901: F, t1588: F, t6454: F, t1317: F, t28: F, t8270: F, t25996: F, t379: F) -> (F, F, F, F, F, F) {
    let t100166 = t92103 * t979;
    let t100174 = t5674 * t7793 * t25899 * t1643;
    let t100178 = t38477 * t1307;
    let t100180 = t1901 * t100178 * t11064;
    let t100182 = t6454 * t1588;
    let t100185 = t1317 * t28 * t8270 * t100182;
    let t100186 = t25996 * t379;
    (t100166, t100174, t100180, t100182, t100185, t100186)
}
