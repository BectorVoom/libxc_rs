//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 506/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk506(t2889: f64, t93: f64, t1880: f64, t1882: f64, t2733: f64, t2736: f64, t1888: f64, t2888: f64, t534: f64, t1896: f64, t452: f64, t1910: f64, t1912: f64, t1914: f64, t1916: f64, t2803: f64, t2807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2890 = t93 * t2889;
    let t2897 = t1880 - 0.9421211958699838_f64 * t2733 + t1882 + 0.9421211958699838_f64 * t2736;
    let t2901 = t2897 * t534 - t1888 * t2888 / 2.0_f64;
    let t2902 = t2901 * t1896;
    let t2903 = t2902 * t452;
    let t2912 = t1910 - 4.0_f64 * t2803 + t1912 + 4.0_f64 * t2807 + t1914 - 0.821419393556371_f64 * t2733 + t1916 + 0.821419393556371_f64 * t2736;
    (t2890, t2897, t2901, t2902, t2903, t2912)
}
