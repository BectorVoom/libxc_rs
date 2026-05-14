//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 866/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk866<F: Float>(t1939: F, t247: F, t1947: F, t713: F, t1971: F, t1979: F, t1915: F, t690: F, t1954: F, t709: F, t2011: F, t2099: F, t757: F, t2020: F, t5712: F, t2032: F) -> (F, F, F, F, F, F, F, F) {
    let t5873 = 1.0 / t1939 / t247;
    let t5877 = t1947 * t713;
    let t5893 = t1971 * t1979;
    let t5897 = t690 * t1915;
    let t5903 = t709 * t1954;
    let t5921 = t2099 * t2011;
    let t5922 = t757 * t5921;
    let t5925 = t2020 * t5712;
    let t5928 = t2099 * t2032;
    (t5873, t5877, t5893, t5897, t5903, t5922, t5925, t5928)
}
