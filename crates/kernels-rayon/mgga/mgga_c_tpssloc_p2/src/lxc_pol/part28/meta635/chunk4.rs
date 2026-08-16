//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2015/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2015(t90898: f64, t90900: f64, t1336: f64, t16206: f64, t27097: f64, t27098: f64, t3777: f64, t3851: f64, t7208: f64, t81037: f64, t81039: f64, t81041: f64, t81043: f64, t81047: f64, t81050: f64, t81061: f64, t81066: f64, t90883: f64, t90887: f64, t90892: f64, t90895: f64) -> f64 {
    let t93562 = 0.3289868133696452873e-1_f64 * t90898;
    let t93563 = 0.52089578783527170489e-1_f64 * t90900;
    let t93567 = -0.38381794893125283518e-1_f64 * t81037 + 0.25587863262083522346e0_f64 * t81039 + 0.38381794893125283518e-1_f64 * t81041 - t1336 * t27097 * t3851 - 0.23029076935875170111e0_f64 * t81043 - 0.10417915756705434098e0_f64 * t81047 + 0.16449340668482264365e-1_f64 * t81050 - t1336 * t7208 * t16206 - 0.3289868133696452873e-1_f64 * t90883 - 0.16449340668482264365e-1_f64 * t90887 - 0.25587863262083522346e0_f64 * t81061 - 0.6579736267392905746e-1_f64 * t90892 + 0.6579736267392905746e-1_f64 * t90895 - t93562 + t93563 - 2.0_f64 * t3777 * t27098 + 0.3289868133696452873e-1_f64 * t81066;
    t93567
}
