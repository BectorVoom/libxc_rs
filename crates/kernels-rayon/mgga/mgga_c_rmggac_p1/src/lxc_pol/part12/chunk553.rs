//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 553/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk553(t118: f64, t7419: f64, t7418: f64, t675: f64, t1253: f64, t1986: f64, t211: f64, t483: f64, t1965: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7420 = t118 * t7419;
    let t7421 = t7418 * t7420;
    let t7422 = t675 * t7421;
    let t7423 = 0.85129199786595678796e-5_f64 * t7422;
    let t7424 = t1986 * t1253;
    let t7425 = t675 * t7424;
    let t7426 = 0.25538759935978703638e-4_f64 * t7425;
    let t7427 = t211 * t483;
    let t7428 = t1965 * t7427;
    (t7421, t7423, t7424, t7426, t7427, t7428)
}
