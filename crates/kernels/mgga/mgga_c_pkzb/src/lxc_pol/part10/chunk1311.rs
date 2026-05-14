//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1311/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1311<F: Float>(t21221: F, t2787: F, t1972: F, t730: F, t9355: F, t5893: F, t9397: F, t1987: F, t9359: F, t1854: F, t3519: F, t1857: F, t1856: F, t3525: F, t5776: F, t25883: F, t25885: F, t25887: F, t25889: F, t25891: F, t25895: F, t25897: F) -> (F, F, F, F, F, F, F) {
    let t25899 = 0.64327917994770140268e2 * t21221 * t2787;
    let t25902 = 0.11696447245269292414e1 * t730 * t9355 * t1972;
    let t25905 = 0.10389515463408878255e3 * t730 * t9397 * t5893;
    let t25907 = 0.69263436422725855036e2 * t1987 * t9359;
    let t25908 = t3519 * t1854;
    let t25910 = 2.0 * t25908 * t1857;
    let t25913 = 24.0 * t5776 * t3525 * t1856;
    let t25914 = -t25883 - t25885 + t25887 + t25889 - t25891 - t25895 - t25897 + t25899 + t25902 + t25905 - t25907 - t25910 - t25913;
    (t25899, t25902, t25905, t25907, t25910, t25913, t25914)
}
