//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3931/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3931<F: Float>(t114: F, t75532: F, t75655: F, t4245: F, t670: F, t10416: F, t1312: F, t13426: F, t13435: F, t13440: F, t13514: F, t1518: F, t18227: F, t18245: F, t21881: F, t2322: F, t2371: F, t27123: F, t4248: F, t4292: F, t49686: F, t5523: F, t5920: F, t60650: F, t60656: F, t61010: F, t75439: F, t75485: F, t75494: F, t7889: F, t93: F) -> (F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t75657 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t75532 + t75655);
    let t75667 = t4245 * t670;
    let t75672 = F::cast_from(2.0_f64) * t10416 * t5920 + F::cast_from(2.0_f64) * t1312 * t75657 + F::cast_from(8.0_f64) * t13426 * t4292 + F::cast_from(4.0_f64) * t13435 * t5920 + F::cast_from(2.0_f64) * t13440 * t5920 + F::cast_from(4.0_f64) * t13514 * t4248 + F::cast_from(4.0_f64) * t13514 * t7889 + F::cast_from(4.0_f64) * t1518 * t49686 + F::cast_from(4.0_f64) * t1518 * t75485 + F::cast_from(8.0_f64) * t1518 * t75667 + F::cast_from(8.0_f64) * t18227 * t4292 + F::cast_from(2.0_f64) * t18245 * t2371 + F::cast_from(4.0_f64) * t21881 * t2322 + F::cast_from(4.0_f64) * t21881 * t5523 + F::cast_from(8.0_f64) * t27123 * t4292 + F::cast_from(4.0_f64) * t670 * t75439 + F::cast_from(4.0_f64) * t75494 * t93 + F::cast_from(2.0_f64) * t60650 + F::cast_from(2.0_f64) * t60656 + t61010;
    (t75657, t75667, t75672)
}
