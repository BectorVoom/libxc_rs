//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2172/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2172(t29894: f64, t3336: f64, t100802: f64, t100806: f64, t106684: f64, t106738: f64, t106786: f64, t106834: f64, t107206: f64, t107257: f64, t107305: f64, t107354: f64, t107405: f64, t107457: f64, t107509: f64, t107557: f64, t107603: f64, t107649: f64, t107691: f64, t107733: f64, t1100: f64, t1102: f64, t1699: f64, t198: f64, t20230: f64, t25709: f64, t25713: f64, t27712: f64, t27717: f64, t336: f64, t5019: f64, t5023: f64, t6396: f64, t6400: f64, t7181: f64, t94142: f64, t94149: f64) -> f64 {
    let t107741 = t29894 * t3336;
    let t107772 = t198 * t336 * (t106684 + t106738 + t106786 + t106834 + t107206 + t107257 + t107305 + t107354 + t107405 + t107457 + t107509 + t107557 + t107603 + t107649 + t107691 + t107733) * t1102 - t5023 * t107741 * t1100 - 2.0_f64 * t5023 * t100802 * t1699 + 4.0_f64 * t5023 * t100806 * t27717 - 2.0_f64 * t5023 * t27712 * t5019 + 2.0_f64 * t5023 * t94142 * t6400 - 6.0_f64 * t5023 * t94149 * t6400 * t1100 + 4.0_f64 * t5023 * t25713 * t1699 * t5019 - t5023 * t25709 * t6396 + 2.0_f64 * t5023 * t25713 * t6396 * t1100 - t5023 * t7181 * t20230;
    t107772
}
