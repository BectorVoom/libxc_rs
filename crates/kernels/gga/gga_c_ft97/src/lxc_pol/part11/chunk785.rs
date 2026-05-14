//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 785/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk785<F: Float>(t37355: F, t37748: F, t37357: F, t419: F, t420: F, t37391: F, t423: F, t1725: F, t8090: F, t37723: F, t37725: F, t37728: F, t37733: F, t37736: F, t37739: F, t37742: F, t37745: F) -> (F, F, F, F) {
    let t37749 = t37748 * t37355;
    let t37752 = t419 * t420 * t37749 * t37357;
    let t37756 = t419 * t420 * t423 * t37391;
    let t37758 = t1725 * t8090;
    let t37760 = 0.49939889221399176955e0 * t37723 + 0.54479879150617283952e0 * t37725 - 0.68099848938271604939e-1 * t37728 - 0.23834947128395061728e0 * t37733 - 0.85124811172839506172e-2 * t37736 - 0.1134997482304526749e-1 * t37739 + 0.85124811172839506172e-2 * t37742 + 0.26483274587105624143e-1 * t37745 + 0.66208186467764060357e-1 * t37752 + 0.6384360837962962963e-2 * t37756 - 0.40859909362962962964e0 * t37758;
    (t37752, t37756, t37758, t37760)
}
