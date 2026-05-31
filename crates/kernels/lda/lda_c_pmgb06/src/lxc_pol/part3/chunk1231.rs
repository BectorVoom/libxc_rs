//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1231/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1231<F: Float>(t2209: F, t384: F, t123: F, t317: F, t4575: F, t740: F, t10599: F, t10603: F, t10606: F, t10609: F, t10614: F, t10617: F, t10620: F, t10623: F, t10635: F, t10640: F, t10643: F, t10646: F, t1316: F, t2258: F, t388: F, t4006: F) -> F {
    let t14617 = t384 * t2209;
    let t14623 = t123 * t740 * t4575 * t317;
    let t14625 = t10599 - t10603 + F::cast_from(0.004067943812504169_f64) * t10606 + F::cast_from(0.012203831437512505_f64) * t10609 - t10614 + t10617 - F::cast_from(0.0002905674151788692_f64) * t10620 - F::cast_from(0.0017434044910732151_f64) * t10623 - F::cast_from(0.002615106736609823_f64) * t10635 + t10640 - t10643 - F::cast_from(0.020146007452401596_f64) * t10646 + F::cast_from(18.0_f64) * t1316 * t2258 * t4006 + F::cast_from(9.0_f64) * t1316 * t388 * t14617 - F::cast_from(0.16213771438917426_f64) * t14623;
    t14625
}
