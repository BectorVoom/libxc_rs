//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1268/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1268(t193: f64, t8418: f64, t200: f64, t8369: f64, t22960: f64, t4255: f64, t7540: f64, t776: f64, t1877: f64, t2219: f64, t8366: f64, t1484: f64, t6665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118436 = t193 * t8418;
    let t118439 = t193 * t200 * t8369;
    let t118440 = t22960 * t4255;
    let t118454 = t7540 * t776;
    let t118455 = t22960 * t118454;
    let t118465 = t1877 * t8366 * t2219;
    let t118466 = t1484 * t6665;
    (t118436, t118439, t118440, t118454, t118455, t118465, t118466)
}
