//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1328/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1328<F: Float>(t3151: F, t357: F, t15907: F, t3117: F, t11883: F, t11888: F, t16037: F, t16040: F, t16045: F, t16049: F, t16052: F, t16057: F, t16062: F, t16064: F, t16067: F, t1656: F, t3115: F, t3241: F, t4887: F, t4896: F, t4902: F) -> F {
    let t16068 = t3151 * t357;
    let t16069 = t15907 * t16068;
    let t16070 = t3117 * t16069;
    let t16073 = -t16037 + F::cast_from(0.14291339372689912324e-3_f64) * t11888 - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t16040 - F::cast_from(0.21437009059034868486e-3_f64) * t3115 * t16045 + F::cast_from(0.22866142996303859718e-2_f64) * t16049 * t4902 - F::cast_from(0.45732285992607719436e-2_f64) * t16052 * t4896 + t16057 + F::new(11.0) / F::new(324.0) * t11883 * t1656 + t16062 - t16064 - t3241 * t4887 / F::new(54.0) + F::cast_from(0.21437009059034868486e-3_f64) * t16067 * t16070;
    t16073
}
