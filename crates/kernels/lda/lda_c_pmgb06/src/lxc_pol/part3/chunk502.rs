//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 502/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk502<F: Float>(t1831: F, t525: F, t103: F, t1474: F, t1563: F, t1571: F, t1572: F, t1818: F, t1823: F, t1828: F, t1833: F, t2060: F, t2077: F, t2079: F, t2082: F) -> (F, F) {
    let t2085 = t525 * t1831;
    let t2088 = t1563 + F::new(0.011997222222222222) * t1474 + F::new(0.011997222222222222) * t1818 - F::new(0.023994444444444443) * t1823 + F::new(0.07198333333333333) * t1828 + F::new(0.07198333333333333) * t1833 + t1571 + F::new(0.0044444444444444444) * t1572 + F::new(0.0044444444444444444) * t2077 - F::new(0.0022222222222222222) * t103 * t2079 + F::new(0.013333333333333334) * t103 * t2082 + F::new(0.013333333333333334) * t2060 * t2085;
    (t2085, t2088)
}
