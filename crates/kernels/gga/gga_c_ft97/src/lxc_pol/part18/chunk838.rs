//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 838/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk838<F: Float>(t1559: F, t22986: F, t7793: F, t446: F, t1882: F, t5693: F, t358: F, t5617: F) -> (F, F, F, F) {
    let t22987 = t22986 * t1559;
    let t22988 = t7793 * t22987;
    let t22989 = t446 * t22988;
    let t22991 = t1882 * t5693;
    let t22993 = t5617 * t358;
    (t22988, t22989, t22991, t22993)
}
