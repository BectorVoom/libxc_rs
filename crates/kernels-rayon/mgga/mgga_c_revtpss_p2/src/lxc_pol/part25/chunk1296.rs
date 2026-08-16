//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1296/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1296(t265: f64, t393: f64, t1100: f64, t1102: f64, t11105: f64, t12190: f64, t198: f64, t25709: f64, t25713: f64, t3329: f64, t3333: f64, t336: f64, t5023: f64, t52188: f64, t7181: f64, t93458: f64, t93514: f64, t93852: f64, t93907: f64, t93958: f64, t94021: f64, t94075: f64, t94131: f64, t94138: f64, t94142: f64, t94149: f64, t94213: f64) -> f64 {
    let t394 = t265 < t393;
    let t94214 = piecewise3(t394, t198 * t336 * (t93458 + t93514 + t93852 + t93907 + t93958 + t94021 + t94075 + t94131) * t1102 - 3.0_f64 * t5023 * t94138 * t1100 + 6.0_f64 * t5023 * t94142 * t3333 - 3.0_f64 * t5023 * t25709 * t3329 - 6.0_f64 * t5023 * t94149 * t11105 + 6.0_f64 * t5023 * t25713 * t52188 - t5023 * t7181 * t12190, t94213);
    t94214
}
