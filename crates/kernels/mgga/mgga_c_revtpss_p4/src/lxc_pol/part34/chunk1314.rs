//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1314/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1314<F: Float>(t1501: F, t5920: F, t1936: F, t30138: F, t7741: F, t30004: F, t4248: F, t22633: F, t93: F, t30143: F, t7889: F, t22589: F, t94982: F) -> (F, F, F, F, F, F, F, F) {
    let t114378 = t1501 * t5920;
    let t114380 = F::cast_from(6.0_f64) * t114378 * t1936;
    let t114382 = F::cast_from(12.0_f64) * t30138 * t7741;
    let t114384 = F::cast_from(6.0_f64) * t4248 * t30004;
    let t114385 = t93 * t22633;
    let t114387 = F::cast_from(2.0_f64) * t114385 * t1936;
    let t114389 = F::cast_from(6.0_f64) * t30143 * t7741;
    let t114391 = F::cast_from(6.0_f64) * t7889 * t30004;
    let t114394 = t94982 * t22589;
    (t114378, t114380, t114382, t114384, t114387, t114389, t114391, t114394)
}
