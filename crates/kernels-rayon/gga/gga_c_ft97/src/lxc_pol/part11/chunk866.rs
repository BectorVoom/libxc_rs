//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 866/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk866(t37355: f64, t37748: f64, t37357: f64, t419: f64, t420: f64, t37391: f64, t423: f64, t1725: f64, t8090: f64, t37723: f64, t37725: f64, t37728: f64, t37733: f64, t37736: f64, t37739: f64, t37742: f64, t37745: f64) -> (f64, f64, f64, f64) {
    let t37749 = t37748 * t37355;
    let t37752 = t419 * t420 * t37749 * t37357;
    let t37756 = t419 * t420 * t423 * t37391;
    let t37758 = t1725 * t8090;
    let t37760 = 0.49939889221399176955e0_f64 * t37723 + 0.54479879150617283952e0_f64 * t37725 - 0.68099848938271604939e-1_f64 * t37728 - 0.23834947128395061728e0_f64 * t37733 - 0.85124811172839506172e-2_f64 * t37736 - 0.1134997482304526749e-1_f64 * t37739 + 0.85124811172839506172e-2_f64 * t37742 + 0.26483274587105624143e-1_f64 * t37745 + 0.66208186467764060357e-1_f64 * t37752 + 0.6384360837962962963e-2_f64 * t37756 - 0.40859909362962962964e0_f64 * t37758;
    (t37752, t37756, t37758, t37760)
}
