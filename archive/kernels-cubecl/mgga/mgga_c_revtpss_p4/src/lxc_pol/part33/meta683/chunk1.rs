//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2241/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2241<F: Float>(t6601: F, t7623: F, t21188: F, t26844: F, t104658: F, t104680: F, t104732: F, t1266: F, t17307: F, t1808: F, t20864: F, t20978: F, t21111: F, t26873: F, t29020: F, t29037: F, t5287: F, t5304: F, t5386: F, t6625: F, t7624: F, t97149: F) -> F {
    let t112179 = t6601 * t7623;
    let t112195 = t26844 * t21188;
    let t112200 = -F::cast_from(0.85748036236139473944e-3_f64) * t97149 * t20978 + F::cast_from(0.1270341277572436651e-3_f64) * t104658 - F::cast_from(0.28582678745379824648e-3_f64) * t112179 * t1266 + F::cast_from(0.17149607247227894789e-2_f64) * t17307 * t7623 * t5386 - F::cast_from(0.1270341277572436651e-2_f64) * t7624 * t21111 - F::cast_from(0.57165357490759649296e-3_f64) * t104732 * t1808 + F::cast_from(0.42874018118069736972e-3_f64) * t26873 * t6625 + F::cast_from(0.95275595817932748826e-3_f64) * t7624 * t20864 + F::cast_from(0.95275595817932748827e-3_f64) * t29037 * t5304 + F::cast_from(0.57165357490759649296e-3_f64) * t112195 - F::cast_from(0.45732285992607719436e-2_f64) * t29020 * t5287 + F::cast_from(0.38110238327173099531e-3_f64) * t104680;
    t112200
}
