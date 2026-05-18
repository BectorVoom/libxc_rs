//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 387/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk387<F: Float>(t178: F, t1848: F, t513: F, t831: F, t432: F, t815: F, t350: F, t810: F, t1438: F, t760: F, t332: F) -> (F, F, F, F, F, F) {
    let t1850 = t1848 * t178 / F::new(30.0);
    let t1852 = t831 * t513 / F::new(30.0);
    let t1854 = t432 * t815 / F::new(30.0);
    let t1856 = t350 * t810;
    let t1858 = t1438 * t760;
    let t1859 = t1858 * t332;
    (t1850, t1852, t1854, t1856, t1858, t1859)
}
