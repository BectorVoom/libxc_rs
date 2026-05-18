//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 988/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk988<F: Float>(t1154: F, t14915: F, t829: F, t10541: F, t10544: F, t10548: F, t1153: F, t14133: F, t14210: F, t14216: F, t14238: F, t14278: F, t14283: F, t14896: F, t14899: F, t14902: F, t14907: F, t14913: F, t3295: F, t3381: F, t348: F, t4602: F, t4607: F, t4638: F, t4643: F, t5111: F) -> F {
    let t14917 = t1154 * t14915 * t829;
    let t14920 = -F::new(0.1857375e-1) * t10544 * t4638 + F::new(0.46434375e-2) * t5111 * t14278 - F::new(0.1857375e-1) * t3381 * t14283 - F::new(0.1857375e-1) * t10544 * t4607 + F::new(0.1857375e-1) * t3381 * t14216 - F::new(0.46434375e-2) * t5111 * t14238 - t10541 + F::new(0.24765e-1) * t14896 * t4643 + F::new(0.9286875e-2) * t14899 * t4602 + F::new(0.619125e-2) * t14902 * t348 - F::new(0.1857375e-1) * t3381 * t14210 - F::new(0.1857375e-1) * t14907 * t3295 + F::new(0.9286875e-2) * t5111 * t14133 - F::new(0.26531111111111111111e-1) * t10548 - F::new(0.35374814814814814814e-1) * t14913 - F::new(0.53062222222222222222e-1) * t1153 * t14917;
    t14920
}
