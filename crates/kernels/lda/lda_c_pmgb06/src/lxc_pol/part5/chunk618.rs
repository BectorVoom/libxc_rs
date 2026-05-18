//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 618/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk618<F: Float>(t1447: F, t1916: F, t1920: F, t1730: F, t871: F, t3213: F, t806: F, t1872: F, t441: F, t1504: F, t831: F, t1848: F, t490: F) -> (F, F, F, F, F, F, F) {
    let t4723 = F::new(8.0) / F::new(135.0) * t1447 * t1916;
    let t4725 = F::new(4.0) / F::new(81.0) * t1447 * t1920;
    let t4740 = t871 * t1730;
    let t4777 = t3213 * t806;
    let t4779 = t441 * t1872;
    let t4786 = F::new(2.0) / F::new(45.0) * t831 * t1504;
    let t4788 = F::new(2.0) / F::new(45.0) * t1848 * t490;
    (t4723, t4725, t4740, t4777, t4779, t4786, t4788)
}
