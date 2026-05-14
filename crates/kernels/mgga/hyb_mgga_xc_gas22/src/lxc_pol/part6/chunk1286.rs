//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1286/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1286<F: Float>(t30137: F, t30150: F, t30164: F, t30177: F, t949: F, t968: F, t21552: F, t4273: F, t21393: F, t21396: F, t21541: F, t25214: F, t25217: F, t25220: F, t29757: F, t29760: F, t29788: F, t387: F) -> (F, F, F) {
    let t30182 = 1.0 * t949 * (t30137 + t30150 + t30164 + t30177) * t968;
    let t30184 = 0.16081979498692535067e2 * t21552 * t4273;
    let t30194 = (t21541 - 0.57685185185185185184e-1 * t21393 + 0.12361111111111111111e-1 * t21396 - 0.57685185185185185187e-1 * t25214 + 0.49444444444444444446e-1 * t25217 - 0.18541666666666666667e-1 * t25220 + 0.12361111111111111111e-1 * t29757 - 0.18541666666666666667e-1 * t29760 + 0.278125e-1 * t29788) * t387;
    (t30182, t30184, t30194)
}
