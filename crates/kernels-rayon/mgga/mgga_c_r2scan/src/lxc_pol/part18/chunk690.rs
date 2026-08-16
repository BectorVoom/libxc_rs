//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 690/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk690(t1835: f64, t712: f64, t1837: f64, t234: f64, t1831: f64, t732: f64, t225: f64, t5317: f64, t739: f64, t166: f64, t1726: f64, t5325: f64, t591: f64) -> (f64, f64, f64, f64, f64) {
    let t5351 = t1835 * t712;
    let t5352 = t5351 * t1837;
    let t5354 = 0.31168546390226634765e3_f64 * t234 * t5352;
    let t5355 = t732 * t1831;
    let t5357 = t225 * t5317;
    let t5358 = t739 * t5357;
    let t5360 = 0.11696447245269292414e1_f64 * t234 * t5358;
    let t5363 = t1726 * t166;
    let t5364 = t5325 * t591;
    (t5354, t5355, t5360, t5363, t5364)
}
