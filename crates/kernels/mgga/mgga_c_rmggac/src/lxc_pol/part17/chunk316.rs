//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 316/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk316<F: Float>(t1810: F, t277: F, t128: F, t1704: F, t793: F, t1773: F, t797: F, t1776: F, t305: F, t1734: F, t1737: F, t838: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1923 = t277 * t1810;
    let t1926 = t128 * t1704;
    let t1927 = t793 * t1926;
    let t1929 = t797 * t1773;
    let t1931 = t305 * t1776;
    let t1933 = t128 * t1734;
    let t1934 = t305 * t1933;
    let t1936 = t128 * t1737;
    let t1937 = t838 * t1936;
    (t1923, t1926, t1927, t1929, t1931, t1933, t1934, t1936, t1937)
}
