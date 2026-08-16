//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 702/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk702<F: Float>(t4529: F, t693: F, t1112: F, t2151: F, t2160: F, t643: F, t2158: F, t638: F, t248: F, t3662: F, t3672: F, t3678: F, t3700: F, t4483: F, t4485: F, t4516: F, t4518: F, t4520: F, t4522: F, t4525: F, t4527: F) -> F {
    let t4531 = F::cast_from(0.0003662289461201309_f64) * t4529 * t693;
    let t4532 = t2151 * t1112;
    let t4534 = t643 * t2160;
    let t4537 = F::cast_from(8.0_f64) * t638 * t2158;
    let t4538 = t4483 - t4485 + t248 * t4516 + F::cast_from(8.0_f64) * t4518 + F::cast_from(12.0_f64) * t4520 + F::cast_from(20.0_f64) * t4522 + t4525 + F::cast_from(0.0004883052614935079_f64) * t3662 - F::cast_from(32.0_f64) * t4527 + t3672 - t3678 + t3700 - t4531 + F::cast_from(0.00024415263074675396_f64) * t4532 - F::cast_from(8.0_f64) * t4534 + t4537;
    t4538
}
