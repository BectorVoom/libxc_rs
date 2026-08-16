//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1172/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1172(t6472: f64, t8652: f64, t8782: f64, t12213: f64, t2417: f64, t3306: f64, t6781: f64, t331: f64, t8703: f64, t2306: f64, t3074: f64, t3075: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t22141 = t6472 * t8652;
    let t22142 = t8782 * t22141;
    let t22172 = t12213 * t2417;
    let t22192 = t6781 * t3306;
    let t22237 = t8703 * t331;
    let t22263 = t3074 * t2306 * t22237;
    let t22334 = t3075 * t837;
    (t22142, t22172, t22192, t22263, t22334)
}
