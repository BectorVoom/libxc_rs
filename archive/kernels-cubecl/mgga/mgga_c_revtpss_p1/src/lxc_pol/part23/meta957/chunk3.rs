//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3204/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3204<F: Float>(t1803: F, t20850: F, t1238: F, t1248: F, t12809: F, t12866: F, t13045: F, t17351: F, t17426: F, t17605: F, t17654: F, t17693: F, t17694: F, t17709: F, t1794: F, t20851: F, t21008: F, t24228: F, t24569: F, t3611: F, t3625: F, t3720: F, t44225: F, t5284: F, t5320: F, t5405: F, t5406: F, t6688: F, t70890: F, t71112: F, t83040: F, t83943: F, t83950: F) -> F {
    let t84098 = t20850 * t1803;
    let t84132 = -F::cast_from(0.64311027177104605458e-3_f64) * t20851 * t5320 + F::cast_from(0.34299214494455789577e-2_f64) * t84098 * t1238 + F::cast_from(0.42874018118069736972e-3_f64) * t12866 * t71112 * t5406 + F::cast_from(0.14291339372689912324e-2_f64) * t17693 * t17694 * t83040 + F::cast_from(0.14291339372689912324e-2_f64) * t17654 * t17694 * t83943 - F::cast_from(0.7145669686344956162e-3_f64) * t17351 * t17694 * t83950 + F::cast_from(0.38586616306262763276e-2_f64) * t17709 * t3720 * t70890 * t13045 * t1794 * t1248 - F::cast_from(0.85748036236139473944e-3_f64) * t17426 * t24569 - F::cast_from(0.3811023832717309953e-2_f64) * t17605 * t21008 - F::cast_from(0.63517063878621832552e-3_f64) * t3625 * t44225 * t24228 * t5405 + F::cast_from(0.12862205435420921092e-2_f64) * t12809 * t3720 * t6688 * t3611 * t5284;
    t84132
}
