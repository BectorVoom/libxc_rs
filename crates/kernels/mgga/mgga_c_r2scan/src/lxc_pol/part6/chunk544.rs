//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 544/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk544<F: Float>(t1923: F, t686: F, t1803: F, t1939: F, t1942: F, t1945: F, t1946: F, t1949: F, t1957: F, t1966: F, t201: F, t207: F, t208: F, t390: F, t664: F, t674: F, t682: F, t687: F, t689: F, t690: F, t705: F) -> (F, F) {
    let t1973 = t686 * t1923;
    let t1976 = -0.11015083824637807018e1 * t390 * t1939 - 0.11696447245269292414e1 * t705 * t1942 - 0.10389515463408878255e3 * t1945 * t1946 - 0.23392894490538584828e1 * t705 * t1949 + 6.0 * t687 * t208 * t1923 - 0.19298375398431042081e3 * t1957 * t690 * t1923 - 4.0 * t674 * t682 * t664 - 2.0 * t674 * t208 * t1966 + 0.20548e0 * t201 * t1966 * t207 + 0.66090502947826842111e1 * t1973 * t689 - t1803;
    (t1973, t1976)
}
