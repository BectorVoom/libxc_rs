//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 597/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk597<F: Float>(t3937: F, t865: F, t191: F, t813: F, t301: F, t467: F, t1680: F, t694: F, t560: F, t1679: F, t811: F, t4: F, t483: F) -> (F, F, F, F, F, F, F) {
    let t3939 = 0.39512695097613069591e1 * t3937 * t865;
    let t3952 = 1.0 / t813 / t191;
    let t3984 = t467 * t301;
    let t3986 = t694 * t1680 * t3984;
    let t3988 = t560 * t3952;
    let t3990 = t1679 * t3988 * t811;
    let t3992 = t483 * t4;
    (t3939, t3952, t3984, t3986, t3988, t3990, t3992)
}
