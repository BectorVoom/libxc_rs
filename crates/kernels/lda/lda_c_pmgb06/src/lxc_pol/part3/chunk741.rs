//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 741/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk741<F: Float>(t2057: F, t405: F, t2054: F, t103: F, t2060: F, t3396: F, t3398: F, t3400: F, t3413: F, t3414: F, t4635: F, t4642: F, t5006: F, t5010: F, t5013: F, t5016: F, t5019: F, t5022: F, t5025: F, t5028: F) -> F {
    let t5032 = F::new(0.017777777777777778) * t405 * t2057;
    let t5034 = F::new(0.002962962962962963) * t405 * t2054;
    let t5038 = F::new(0.057777777777777775) * t5006 - F::new(0.015996296296296297) * t4635 + F::new(0.2639388888888889) * t4642 - t3413 - t3414 + F::new(0.013333333333333334) * t103 * t5010 - F::new(0.04) * t103 * t5013 - F::new(0.0022222222222222222) * t103 * t5016 - F::new(0.002962962962962963) * t103 * t5019 - F::new(0.008888888888888889) * t2060 * t5022 + F::new(0.013333333333333334) * t103 * t5025 + F::new(0.05333333333333334) * t2060 * t5028 - t5032 + t5034 - F::new(0.014814814814814815) * t3396 + F::new(0.0044444444444444444) * t3398 + F::new(0.0014814814814814814) * t3400;
    t5038
}
