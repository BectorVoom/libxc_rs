//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 723/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk723<F: Float>(t445: F, t4779: F, t439: F, t1427: F, t2002: F, t1504: F, t831: F, t1848: F, t490: F, t4738: F, t4739: F, t4740: F, t4756: F, t4759: F, t4764: F, t4769: F, t4771: F, t4774: F, t4776: F, t4778: F) -> (F, F, F, F, F, F) {
    let t4780 = t4779 * t445;
    let t4782 = F::new(2.0) / F::new(45.0) * t439 * t4780;
    let t4784 = F::new(2.0) / F::new(45.0) * t2002 * t1427;
    let t4786 = F::new(2.0) / F::new(45.0) * t831 * t1504;
    let t4788 = F::new(2.0) / F::new(45.0) * t1848 * t490;
    let t4789 = -t4738 - t4739 + F::cast_from(0.033245444444444446_f64) * t4740 + t4756 - t4759 - t4764 - t4769 - t4771 - t4774 - t4776 - t4778 + t4782 + t4784 + t4786 + t4788;
    (t4780, t4782, t4784, t4786, t4788, t4789)
}
