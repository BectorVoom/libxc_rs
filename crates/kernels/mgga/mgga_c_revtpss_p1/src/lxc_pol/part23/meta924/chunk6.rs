//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2994/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2994<F: Float>(t341: F, t79366: F, t79386: F, t54397: F, t78900: F, t15689: F, t15700: F, t15745: F, t19993: F, t225: F, t3095: F, t366: F, t375: F, t4893: F, t53320: F, t53328: F, t53728: F, t53876: F, t53901: F, t53955: F, t6278: F, t66093: F, t66139: F, t66141: F, t66155: F, t66158: F, t66176: F, t66215: F, t66218: F, t66221: F, t66542: F, t66777: F, t77513: F) -> (F, F, F) {
    let t79388 = (t79366 + t79386) * t341;
    let t79395 = t78900 * t54397;
    let t79407 = F::cast_from(0.42874018118069736972e-3_f64) * t66093 - F::cast_from(0.42874018118069736972e-3_f64) * t15689 * t66777 * t4893 * t3095 - F::cast_from(0.22866142996303859718e-2_f64) * t66139 - F::cast_from(0.14291339372689912324e-3_f64) * t66141 + t66155 / F::cast_from(48.0_f64) - t66158 / F::cast_from(72.0_f64) + t53876 + t53320 * t53328 * t77513 / F::cast_from(16.0_f64) + F::cast_from(0.21437009059034868486e-3_f64) * t79388 * t225 * t366 * t375 - F::cast_from(0.95275595817932748825e-4_f64) * t53901 - F::cast_from(0.57165357490759649295e-3_f64) * t66176 + F::cast_from(0.25724410870841842183e-2_f64) * t15700 * t53728 * t79395 - F::cast_from(0.17149607247227894789e-2_f64) * t66542 * t19993 + F::cast_from(0.34299214494455789578e-2_f64) * t15745 * t6278 - F::cast_from(0.95275595817932748825e-4_f64) * t53955 - t66215 / F::cast_from(81.0_f64) - t66218 / F::cast_from(324.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t66221;
    (t79388, t79395, t79407)
}
