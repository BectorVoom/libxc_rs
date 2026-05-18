//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 475/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk475<F: Float>(t1925: F, t493: F, t435: F, t814: F, t132: F, t436: F, t802: F, t489: F, t843: F, t161: F, t490: F, t831: F) -> (F, F, F, F, F, F, F) {
    let t1927 = t493 * t1925 / F::new(45.0);
    let t1928 = t435 * t814;
    let t1929 = t132 * t1928;
    let t1930 = t1929 / F::new(45.0);
    let t1931 = t802 * t436;
    let t1932 = t1931 / F::new(45.0);
    let t1933 = t489 * t843;
    let t1934 = t161 * t1933;
    let t1935 = t1934 / F::new(45.0);
    let t1936 = t831 * t490;
    (t1927, t1928, t1930, t1932, t1933, t1935, t1936)
}
