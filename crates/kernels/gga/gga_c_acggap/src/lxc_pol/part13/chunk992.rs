//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 992/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk992<F: Float>(t31168: F, t13299: F, t31057: F, t35288: F, t4643: F, t7486: F, t2095: F, t1427: F, t31491: F, t7381: F, t1345: F, t1983: F, t7380: F, t1462: F, t7614: F, t1446: F, t7605: F) -> (F, F, F, F, F, F, F, F) {
    let t35373 = 0.14291339372689912324e-2 * t31168;
    let t35379 = t31057 * t13299 * t35288;
    let t35380 = 0.31448092289604152068e-3 * t35379;
    let t35383 = t4643 * t7486;
    let t35384 = t2095 * t35383;
    let t35385 = 0.305625e-1 * t35384;
    let t35387 = t31491 * t7381 * t1427;
    let t35388 = t35387 / 8.0;
    let t35390 = t7380 * t1983 * t1345;
    let t35391 = t35390 / 32.0;
    let t35392 = t7614 * t1462;
    let t35393 = 0.24009450146119052704e-1 * t35392;
    let t35394 = t7605 * t1446;
    (t35373, t35380, t35383, t35385, t35388, t35391, t35393, t35394)
}
