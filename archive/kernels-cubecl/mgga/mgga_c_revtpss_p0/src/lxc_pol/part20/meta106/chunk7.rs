//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 610/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk610<F: Float>(t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t2935: F, t2938: F, t2943: F, t2945: F, t2963: F, t2968: F, t2971: F, t2980: F, t2982: F, t2987: F, t2989: F, t3007: F, t3012: F, t3015: F, t311: F, t946: F, t955: F, t965: F, t974: F) -> F {
    let t3018 = -F::cast_from(0.310907e-1_f64) * t2935 * t311 + F::cast_from(2.0_f64) * t2938 * t955 - F::cast_from(2.0_f64) * t2943 * t2945 + F::cast_from(1.0_f64) * t946 * t2963 + F::cast_from(0.32163958997385070134e2_f64) * t2968 * t2971 + t2868 - t2871 + t2878 - t2921 - t2929 - F::cast_from(0.19751673498613801407e-1_f64) * t2980 + F::cast_from(0.11696447245269292414e1_f64) * t2982 * t974 - F::cast_from(0.11696447245269292414e1_f64) * t2987 * t2989 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t3007 + F::cast_from(0.17315859105681463759e2_f64) * t3012 * t3015;
    t3018
}
