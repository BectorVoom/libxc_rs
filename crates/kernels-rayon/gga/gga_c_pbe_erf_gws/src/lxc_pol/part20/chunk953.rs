//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 953/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk953(t3403: f64, t4913: f64, t7580: f64, t3493: f64, t663: f64, t1022: f64, t209: f64, t184: f64, t2737: f64, t1031: f64, t617: f64, t1024: f64) -> (f64, f64, f64, f64, f64) {
    let t10738 = 16.0_f64 / 45.0_f64 * t4913 * t3403;
    let t10739 = 16.0_f64 / 405.0_f64 * t7580;
    let t10741 = 4.0_f64 / 15.0_f64 * t3493 * t663;
    let t10742 = t1022 * t209;
    let t10743 = t10742 * t184;
    let t10745 = 8.0_f64 / 15.0_f64 * t10743 * t2737;
    let t10746 = t617 * t1031;
    let t10747 = t10746 * t184;
    let t10749 = 8.0_f64 / 15.0_f64 * t10747 * t1024;
    (t10738, t10739, t10741, t10745, t10749)
}
