//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3204/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3204(t1803: f64, t20850: f64, t1238: f64, t1248: f64, t12809: f64, t12866: f64, t13045: f64, t17351: f64, t17426: f64, t17605: f64, t17654: f64, t17693: f64, t17694: f64, t17709: f64, t1794: f64, t20851: f64, t21008: f64, t24228: f64, t24569: f64, t3611: f64, t3625: f64, t3720: f64, t44225: f64, t5284: f64, t5320: f64, t5405: f64, t5406: f64, t6688: f64, t70890: f64, t71112: f64, t83040: f64, t83943: f64, t83950: f64) -> f64 {
    let t84098 = t20850 * t1803;
    let t84132 = -0.64311027177104605458e-3_f64 * t20851 * t5320 + 0.34299214494455789577e-2_f64 * t84098 * t1238 + 0.42874018118069736972e-3_f64 * t12866 * t71112 * t5406 + 0.14291339372689912324e-2_f64 * t17693 * t17694 * t83040 + 0.14291339372689912324e-2_f64 * t17654 * t17694 * t83943 - 0.7145669686344956162e-3_f64 * t17351 * t17694 * t83950 + 0.38586616306262763276e-2_f64 * t17709 * t3720 * t70890 * t13045 * t1794 * t1248 - 0.85748036236139473944e-3_f64 * t17426 * t24569 - 0.3811023832717309953e-2_f64 * t17605 * t21008 - 0.63517063878621832552e-3_f64 * t3625 * t44225 * t24228 * t5405 + 0.12862205435420921092e-2_f64 * t12809 * t3720 * t6688 * t3611 * t5284;
    t84132
}
