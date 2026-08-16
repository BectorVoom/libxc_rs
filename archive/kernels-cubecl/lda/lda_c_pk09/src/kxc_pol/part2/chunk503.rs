//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 503/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk503<F: Float>(t2791: F, t2859: F, t1802: F, t1830: F, t1834: F, t1840: F, t1855: F, t1879: F, t1929: F, t1933: F, t1952: F, t2769: F, t2772: F, t2779: F, t2817: F, t444: F, t455: F, t552: F) -> (F, F) {
    let t2860 = t2791 + t2859;
    let t2863 = F::cast_from(1.8805371096875316_f64) * t2769 * t455 - F::cast_from(3.7610742193750633_f64) * t2772 * t455 - F::cast_from(1.8805371096875316_f64) * t2779 * t552 + t444 * t2860 + t1802 - t1830 + t1834 + t1840 + t1855 - t1879 - t1929 - t1933 + t1952 - F::cast_from(22.07984838129906_f64) * t2817;
    (t2860, t2863)
}
