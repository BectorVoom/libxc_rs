//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 674/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk674(t1550: f64, t9765: f64, t2298: f64, t5055: f64, t1856: f64, t194: f64, t201: f64, t1979: f64, t1982: f64, t2320: f64, t8676: f64, t128: f64, t1907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9766 = t1550 * t9765;
    let t9770 = t5055 * t2298;
    let t9774 = t194 * t1856;
    let t9775 = t9774 * t201;
    let t9777 = t9775 * t1979 * t1982;
    let t9779 = t8676 * t2320;
    let t9781 = t128 * t1907;
    (t9766, t9770, t9774, t9775, t9777, t9779, t9781)
}
