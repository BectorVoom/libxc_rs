//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 514/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk514<F: Float>(t1868: F, t473: F, t103: F, t1523: F, t1607: F, t1614: F, t1615: F, t1856: F, t1861: F, t1866: F, t1870: F, t2052: F, t2054: F, t2057: F, t2060: F) -> (F, F) {
    let t2061 = t473 * t1868;
    let t2064 = t1607 + F::new(0.011997222222222222) * t1523 + F::new(0.011997222222222222) * t1856 - F::new(0.023994444444444443) * t1861 + F::new(0.07198333333333333) * t1866 - F::new(0.07198333333333333) * t1870 + t1614 + F::new(0.0044444444444444444) * t1615 + F::new(0.0044444444444444444) * t2052 - F::new(0.0022222222222222222) * t103 * t2054 + F::new(0.013333333333333334) * t103 * t2057 - F::new(0.013333333333333334) * t2060 * t2061;
    (t2061, t2064)
}
