//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2519/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2519<F: Float>(t4477: F, t9292: F, t14472: F, t2439: F, t887: F, t11044: F, t14485: F, t15014: F, t9303: F, t10510: F, t14987: F, t10982: F, t1568: F, t9646: F) -> (F, F, F, F, F, F) {
    let t51213 = t9292 * t4477;
    let t51216 = t2439 * t14472 * t887;
    let t51217 = F::cast_from(0.19514881078765566038e-2_f64) * t51216;
    let t51233 = t11044 * t14485;
    let t51234 = F::cast_from(0.39029762157531132076e-1_f64) * t51233;
    let t51237 = t9303 * t15014;
    let t51239 = t14987 * t10510;
    let t51240 = F::cast_from(0.39029762157531132076e-1_f64) * t51239;
    let t51246 = t9646 * t1568 * t10982;
    (t51213, t51217, t51234, t51237, t51240, t51246)
}
