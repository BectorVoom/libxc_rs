//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1302/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1302(t265: f64, t393: f64, t100806: f64, t107741: f64, t1102: f64, t113678: f64, t113728: f64, t113774: f64, t113819: f64, t113867: f64, t113912: f64, t113961: f64, t114009: f64, t114089: f64, t1699: f64, t198: f64, t23571: f64, t24186: f64, t25713: f64, t27712: f64, t336: f64, t5023: f64, t6396: f64, t6400: f64, t7181: f64, t94149: f64) -> f64 {
    let t394 = t265 < t393;
    let t114090 = piecewise3(t394, t198 * t336 * (t113678 + t113728 + t113774 + t113819 + t113867 + t113912 + t113961 + t114009) * t1102 - 3.0_f64 * t5023 * t107741 * t1699 + 6.0_f64 * t5023 * t100806 * t6400 - 3.0_f64 * t5023 * t27712 * t6396 - 6.0_f64 * t5023 * t94149 * t23571 + 6.0_f64 * t5023 * t25713 * t1699 * t6396 - t5023 * t7181 * t24186, t114089);
    t114090
}
