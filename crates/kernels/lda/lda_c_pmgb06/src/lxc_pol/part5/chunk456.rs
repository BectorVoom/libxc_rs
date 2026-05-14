//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 456/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk456<F: Float>(t1798: F, t183: F, t539: F, t794: F, t188: F, t27: F, t856: F, t545: F, t1404: F, t1412: F, t1918: F, t1922: F, t1927: F, t1930: F, t1932: F, t1935: F, t1937: F, t1938: F, t1965: F, t1971: F, t1974: F, t1976: F) -> (F, F, F, F, F, F) {
    let t2342 = t1798 * t183;
    let t2345 = t794 * t539;
    let t2346 = t2345 * t188;
    let t2349 = t856 * t27;
    let t2350 = t2349 * t545;
    let t2352 = -t1918 + t1922 - t1927 + t1930 + t1932 + t1935 + t1937 - t1938 + 4.0 / 3.0 * t2342 * t188 + 4.0 / 3.0 * t2346 + 4.0 / 3.0 * t1404 + t1412 + 0.10821041362364843 * t2350 + t1965 + t1971 + t1974 + t1976;
    (t2342, t2345, t2346, t2349, t2350, t2352)
}
