//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2932/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2932<F: Float>(t15817: F, t3173: F, t16158: F, t3188: F, t1063: F, t15193: F, t247: F, t3109: F, t11233: F, t11656: F, t12026: F, t15707: F, t15791: F, t15830: F, t15834: F, t15952: F, t3106: F, t3177: F, t3184: F, t42391: F, t4825: F, t4834: F) -> F {
    let t53353 = t15817 * t3173;
    let t53359 = t3188 * t16158;
    let t53363 = t1063 * t247 * t3109 * t15193;
    let t53377 = F::cast_from(0.85748036236139473944e-3_f64) * t53353 - F::cast_from(0.22866142996303859718e-2_f64) * t15830 * t3177 - F::cast_from(0.3811023832717309953e-2_f64) * t15830 * t3184 + F::cast_from(0.57165357490759649295e-3_f64) * t53359 + F::cast_from(0.28582678745379824648e-3_f64) * t53363 - F::cast_from(0.42874018118069736972e-3_f64) * t42391 * t4825 + F::cast_from(0.91464571985215438873e-2_f64) * t3106 * t15791 - F::cast_from(0.76220476654346199061e-2_f64) * t3106 * t15834 - F::cast_from(0.42874018118069736972e-3_f64) * t15707 * t12026 + F::cast_from(0.45732285992607719436e-2_f64) * t11656 * t15952 - F::cast_from(0.85748036236139473944e-3_f64) * t4834 * t11233;
    t53377
}
