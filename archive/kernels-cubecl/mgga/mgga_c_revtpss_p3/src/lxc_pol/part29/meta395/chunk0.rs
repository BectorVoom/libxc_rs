//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1414/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1414<F: Float>(t2710: F, t2713: F, t4371: F, t4353: F, t808: F, t10744: F, t10905: F, t4442: F, t4457: F, t775: F, t800: F, t1548: F, t2430: F) -> (F, F, F, F, F) {
    let t14817 = t2710 * t2713 * t4371;
    let t14819 = t808 * t4353;
    let t14820 = t10744 * t14819;
    let t14823 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t10905 * t4442;
    let t14825 = t800 * t4457 * t775;
    let t14829 = t800 * t1548 * t2430;
    (t14817, t14820, t14823, t14825, t14829)
}
