//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 506/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk506<F: Float>(t2889: F, t93: F, t1880: F, t1882: F, t2733: F, t2736: F, t1888: F, t2888: F, t534: F, t1896: F, t452: F, t1910: F, t1912: F, t1914: F, t1916: F, t2803: F, t2807: F) -> (F, F, F, F, F, F) {
    let t2890 = t93 * t2889;
    let t2897 = t1880 - F::cast_from(0.9421211958699838_f64) * t2733 + t1882 + F::cast_from(0.9421211958699838_f64) * t2736;
    let t2901 = t2897 * t534 - t1888 * t2888 / F::new(2.0);
    let t2902 = t2901 * t1896;
    let t2903 = t2902 * t452;
    let t2912 = t1910 - F::new(4.0) * t2803 + t1912 + F::new(4.0) * t2807 + t1914 - F::cast_from(0.821419393556371_f64) * t2733 + t1916 + F::cast_from(0.821419393556371_f64) * t2736;
    (t2890, t2897, t2901, t2902, t2903, t2912)
}
