//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3182/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3182<F: Float>(t5245: F, t6587: F, t20820: F, t5265: F, t20851: F, t5362: F, t1042: F, t1121: F, t1261: F, t17505: F, t17654: F, t1794: F, t1808: F, t20907: F, t20914: F, t21272: F, t247: F, t3604: F, t3719: F, t5384: F, t5391: F, t5397: F, t58983: F, t606: F, t69723: F, t70756: F, t70758: F, t71300: F, t78785: F) -> (F, F) {
    let t83567 = t5245 * t6587;
    let t83580 = t20820 * t5265;
    let t83584 = t20851 * t5362;
    let t83592 = -F::cast_from(0.14481890564325777821e-1_f64) * t69723 * t1808 - F::cast_from(0.14481890564325777821e-1_f64) * t21272 * t5397 + F::cast_from(0.12862205435420921092e-2_f64) * t5384 * t247 * t3719 * t83567 - F::cast_from(0.14481890564325777821e-1_f64) * t70756 + F::cast_from(0.28582678745379824648e-3_f64) * t70758 + F::cast_from(0.45732285992607719436e-2_f64) * t5391 * t20907 + F::cast_from(0.23289590088828005269e-2_f64) * t1261 * t1042 * t58983 * t78785 + F::cast_from(0.42874018118069736972e-3_f64) * t83580 - F::cast_from(0.45732285992607719436e-2_f64) * t17505 * t20914 - F::cast_from(0.42874018118069736972e-3_f64) * t83584 - F::cast_from(0.85748036236139473947e-3_f64) * t17654 * t71300 * t3604 * t1794 * t1121 * t606;
    (t83567, t83592)
}
