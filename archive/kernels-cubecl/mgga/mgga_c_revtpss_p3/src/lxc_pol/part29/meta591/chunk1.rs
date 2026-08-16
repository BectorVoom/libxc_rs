//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1964/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1964<F: Float>(t102244: F, t94669: F, t102225: F, t102237: F, t102239: F, t102241: F, t7528: F, t96243: F, t96246: F, t96249: F, t96253: F, t96257: F, t96260: F, t96262: F, t96265: F, t98050: F) -> F {
    let t102246 = F::cast_from(0.15421710918628844644e0_f64) * t94669 * t102244;
    let t102248 = -F::cast_from(0.3427046870806409921e-2_f64) * t102225 - F::cast_from(0.14456046980341999104e-1_f64) * t96243 - F::cast_from(0.34270468708064099208e-1_f64) * t96246 + F::cast_from(0.12851425765524037203e-1_f64) * t96249 - F::cast_from(0.13009920719177044025e-2_f64) * t96253 + F::cast_from(0.8673628188205199462e0_f64) * t98050 * t7528 + t102237 - t102239 + t102241 - t96257 - F::cast_from(0.45699670022203476294e-2_f64) * t96260 - F::cast_from(0.12851425765524037203e-1_f64) * t96262 - t102246 - F::cast_from(0.68540937416128198416e-1_f64) * t96265;
    t102248
}
