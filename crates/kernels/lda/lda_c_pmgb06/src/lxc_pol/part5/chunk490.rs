//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 490/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk490<F: Float>(t178: F, t2563: F, t831: F, t844: F, t1550: F, t1557: F, t1712: F, t2027: F, t2030: F, t2032: F, t2534: F, t2535: F, t2536: F, t2557: F, t1438: F, t2377: F) -> (F, F, F, F) {
    let t2565 = t2563 * t178 / 30.0;
    let t2567 = t831 * t844 / 15.0;
    let t2568 = t2534 - t2535 - t2536 + 2.0 / 3.0 * t2027 + 0.12155555555555556 * t2030 + 4.0 / 9.0 * t2032 - t1550 - t1557 + t1712 + t2557 + t2565 + t2567;
    let t2570 = t1438 * t2377;
    (t2565, t2567, t2568, t2570)
}
