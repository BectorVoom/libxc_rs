//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3185/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3185(t5284: f64, t6573: f64, t1121: f64, t1248: f64, t1250: f64, t12787: f64, t12839: f64, t12910: f64, t17401: f64, t17429: f64, t17605: f64, t17729: f64, t20297: f64, t20795: f64, t20957: f64, t21014: f64, t21219: f64, t21223: f64, t21300: f64, t24736: f64, t3626: f64, t3720: f64, t4186: f64, t44535: f64, t5297: f64, t5331: f64, t5333: f64, t5340: f64, t57265: f64, t58920: f64, t59001: f64, t82481: f64, t82859: f64, t82886: f64) -> (f64, f64) {
    let t83662 = t6573 * t5284;
    let t83683 = -0.21437009059034868486e-3_f64 * t5331 * t3720 * t82859 * t5333 + 0.51448821741683684368e-2_f64 * t58920 * t3720 * t82886 * t44535 * t1248 - 0.77173232612525526552e-2_f64 * t59001 * t3720 * t82886 * t20957 - 0.42874018118069736972e-2_f64 * t17729 * t12787 * t20297 * t82481 - 0.64311027177104605458e-3_f64 * t17401 * t21300 + 0.12862205435420921092e-2_f64 * t12910 * t3720 * t83662 * t1250 + 0.25724410870841842183e-2_f64 * t57265 * t3626 * t6573 * t1121 * t5297 - 0.64311027177104605458e-3_f64 * t17429 * t24736 + 0.22866142996303859718e-2_f64 * t17605 * t21219 + 0.45732285992607719436e-2_f64 * t21014 * t21223 - 0.85748036236139473944e-3_f64 * t5340 * t3626 * t20795 * t12839 * t4186;
    (t83662, t83683)
}
