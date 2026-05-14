//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 375/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk375<F: Float>(t1930: F, t736: F, t1871: F, t732: F, t1894: F, t719: F, t735: F, t1757: F, t642: F, t734: F, t1862: F, t716: F, t740: F, t748: F, t1872: F, t747: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1931 = t1930 * sigma2;
    let t1932 = t1931 * t736;
    let t1934 = t732 * t1871;
    let t1935 = t1934 * sigma2;
    let t1936 = t719 * t1894;
    let t1937 = t735 * t1936;
    let t1938 = t1935 * t1937;
    let t1940 = t642 * t1757;
    let t1941 = t735 * t1940;
    let t1942 = t734 * t1941;
    let t1944 = t1862 * t716;
    let t1945 = t1944 * t740;
    let t1946 = t1945 * t748;
    let t1948 = t1872 * t740;
    let t1949 = t747 * t1894;
    (t1931, t1932, t1934, t1935, t1937, t1938, t1941, t1942, t1944, t1945, t1946, t1948, t1949)
}
