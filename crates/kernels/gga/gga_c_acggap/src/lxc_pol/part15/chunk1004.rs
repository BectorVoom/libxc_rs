//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1004/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1004<F: Float>(t31168: F, t13299: F, t31057: F, t35288: F, t4643: F, t7486: F, t2095: F, t1427: F, t31491: F, t7381: F, t1345: F, t1983: F, t7380: F) -> (F, F, F, F, F, F) {
    let t35373 = F::new(0.14291339372689912324e-2) * t31168;
    let t35379 = t31057 * t13299 * t35288;
    let t35383 = t4643 * t7486;
    let t35384 = t2095 * t35383;
    let t35387 = t31491 * t7381 * t1427;
    let t35390 = t7380 * t1983 * t1345;
    (t35373, t35379, t35383, t35384, t35387, t35390)
}
