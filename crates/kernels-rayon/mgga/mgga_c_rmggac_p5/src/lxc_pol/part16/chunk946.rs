//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 946/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk946(t1986: f64, t6590: f64, t675: f64, t2289: f64, t9087: f64, t2412: f64, t8592: f64, t2410: f64, t3350: f64, t8515: f64, t8519: f64, t39277: f64, t8668: f64) -> (f64, f64, f64, f64, f64) {
    let t45742 = t675 * t1986 * t6590;
    let t45744 = t9087 * t2289;
    let t45746 = t2412 * t8592;
    let t45750 = t2410 * t8515 * t3350 * t8519;
    let t45752 = t39277 * t8668;
    (t45742, t45744, t45746, t45750, t45752)
}
