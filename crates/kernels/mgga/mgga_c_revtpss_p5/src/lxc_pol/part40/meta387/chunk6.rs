//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1396/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1396<F: Float>(t1737: F, t3451: F, t1160: F, t5117: F, t1170: F, t12511: F, t12553: F, t16809: F, t16832: F, t16837: F, t16839: F, t16842: F, t16844: F, t16846: F, t16945: F, t16998: F, t17020: F, t3454: F, t435: F, t5125: F) -> F {
    let t17023 = t1737 * t3451;
    let t17026 = t5117 * t1160;
    let t17029 = F::cast_from(0.10254018858216406658e4_f64) * t12553 * t16998 - F::cast_from(4.0_f64) * t12511 * t5125 + t16809 - F::cast_from(0.19751673498613801407e-1_f64) * t16832 - F::cast_from(0.310907e-1_f64) * t17020 * t435 - t16837 - t16839 - t16842 - t16844 - t16846 - t16945 - F::cast_from(2.0_f64) * t17023 * t3454 + F::cast_from(2.0_f64) * t17026 * t1170;
    t17029
}
