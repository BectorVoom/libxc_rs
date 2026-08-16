//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1322/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1322<F: Float>(t114149: F, t114199: F, t2014: F, t30111: F, t5542: F, t101473: F, t29498: F, t29502: F, t4248: F, t22483: F, t7934: F, t1497: F, t29547: F, t77: F) -> (F, F, F, F, F, F) {
    let t114200 = t114149 + t114199;
    let t114216 = F::cast_from(3.0_f64) * t2014 * t30111 * t5542;
    let t114221 = F::cast_from(18.0_f64) * t2014 * t101473 * t29498;
    let t114230 = F::cast_from(12.0_f64) * t4248 * t29502;
    let t114238 = F::cast_from(3.0_f64) * t2014 * t7934 * t22483;
    let t114246 = t77 * t29547 * t1497;
    (t114200, t114216, t114221, t114230, t114238, t114246)
}
