//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 891/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk891<F: Float>(t300: F, t6212: F, t6185: F, t1642: F, t4719: F, t2986: F, t6189: F, t973: F, t981: F, t6205: F, t964: F, t3011: F) -> (F, F, F, F, F, F, F, F) {
    let t6213 = t300 * t6212;
    let t6215 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t6185;
    let t6217 = F::cast_from(0.11696447245269292414e1_f64) * t4719 * t1642;
    let t6219 = t2986 * t6189 * t973;
    let t6221 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t6219;
    let t6223 = t964 * t6205 * t973;
    let t6225 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t6223;
    let t6226 = t3011 * t6189;
    (t6213, t6215, t6217, t6219, t6221, t6223, t6225, t6226)
}
