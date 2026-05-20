//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3185/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3185<F: Float>(t5284: F, t6573: F, t1121: F, t1248: F, t1250: F, t12787: F, t12839: F, t12910: F, t17401: F, t17429: F, t17605: F, t17729: F, t20297: F, t20795: F, t20957: F, t21014: F, t21219: F, t21223: F, t21300: F, t24736: F, t3626: F, t3720: F, t4186: F, t44535: F, t5297: F, t5331: F, t5333: F, t5340: F, t57265: F, t58920: F, t59001: F, t82481: F, t82859: F, t82886: F) -> (F, F) {
    let t83662 = t6573 * t5284;
    let t83683 = -F::cast_from(0.21437009059034868486e-3_f64) * t5331 * t3720 * t82859 * t5333 + F::cast_from(0.51448821741683684368e-2_f64) * t58920 * t3720 * t82886 * t44535 * t1248 - F::cast_from(0.77173232612525526552e-2_f64) * t59001 * t3720 * t82886 * t20957 - F::cast_from(0.42874018118069736972e-2_f64) * t17729 * t12787 * t20297 * t82481 - F::cast_from(0.64311027177104605458e-3_f64) * t17401 * t21300 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t83662 * t1250 + F::cast_from(0.25724410870841842183e-2_f64) * t57265 * t3626 * t6573 * t1121 * t5297 - F::cast_from(0.64311027177104605458e-3_f64) * t17429 * t24736 + F::cast_from(0.22866142996303859718e-2_f64) * t17605 * t21219 + F::cast_from(0.45732285992607719436e-2_f64) * t21014 * t21223 - F::cast_from(0.85748036236139473944e-3_f64) * t5340 * t3626 * t20795 * t12839 * t4186;
    (t83662, t83683)
}
