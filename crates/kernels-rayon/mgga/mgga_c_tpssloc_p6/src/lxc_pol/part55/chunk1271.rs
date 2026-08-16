//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1271/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1271(t2144: f64, t8034: f64, t34339: f64, t85639: f64, t117803: f64, t117809: f64, t117813: f64, t117823: f64, t117834: f64, t117838: f64, t1653: f64, t2128: f64, t24589: f64, t24601: f64, t27432: f64, t27433: f64, t27437: f64, t27549: f64, t27751: f64, t27775: f64, t27820: f64, t32482: f64, t32510: f64, t32515: f64, t32529: f64, t34338: f64, t4930: f64, t4936: f64, t5089: f64, t7283: f64, t7287: f64, t86415: f64, t8871: f64, t94378: f64, t94558: f64) -> f64 {
    let t125148 = t8034 * t2144;
    let t125165 = t85639 * t34339;
    let t125182 = 0.54831135561607547883e-2_f64 * t24589 * t24601 * t117813 * t1653 + 0.54831135561607547883e-2_f64 * t24589 * t86415 * t34338 + 0.54831135561607547883e-2_f64 * t24589 * t117809 * t27433 + 0.54831135561607547883e-2_f64 * t24589 * t125148 * t7287 + 0.16449340668482264365e-1_f64 * t2128 * t4936 * t32515 + 0.54831135561607547883e-2_f64 * t24589 * t117809 * t27437 - 0.10966227112321509577e-1_f64 * t24589 * t94378 * t117803 * t27432 + 0.73108180748810063844e-2_f64 * t27549 * t117809 * t27775 + 0.18277045187202515961e-2_f64 * t125165 - 0.16449340668482264365e-1_f64 * t7283 * t27751 * t32529 - 0.16449340668482264365e-1_f64 * t7283 * t94558 * t8871 + 0.54831135561607547883e-2_f64 * t117823 - t32482 * t5089 - 0.54831135561607547883e-2_f64 * t117834 + 0.16449340668482264365e-1_f64 * t7283 * t4930 * t32515 - t117838 - 0.3289868133696452873e-1_f64 * t2128 * t27820 * t32510;
    t125182
}
