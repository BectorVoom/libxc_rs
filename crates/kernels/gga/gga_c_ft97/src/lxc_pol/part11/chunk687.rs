//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 687/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk687<F: Float>(t2579: F, t684: F, t10007: F, t2603: F, t8392: F, t3892: F, t9853: F, t3891: F, t2526: F, t713: F, t729: F, t762: F, t10000: F, t10004: F, t1901: F, t193: F, t446: F, t89: F, t9845: F, t9850: F, t9855: F, t9976: F, t9982: F, t9985: F, t9989: F, t9993: F, t9997: F) -> (F, F, F, F, F, F, F) {
    let t10008 = t2579 * t684;
    let t10009 = t10007 * t10008;
    let t10012 = t8392 * t2603;
    let t10014 = t3892 * t9853;
    let t10015 = t3891 * t10014;
    let t10018 = t2526 * t713;
    let t10020 = t729 * t762 * t10018;
    let t10022 = 2.0 * t446 * t9845 + t1901 * t9850 / 3.0 + 2.0 / 3.0 * t1901 * t9855 + t89 * t193 * t9976 / 3.0 - t9982 + t1901 * t9985 / 3.0 - t446 * t9989 / 3.0 - 2.0 / 9.0 * t446 * t9993 - t9997 / 3.0 + 4.0 / 9.0 * t10000 + 2.0 * t446 * t10004 - 2.0 / 3.0 * t1901 * t10009 - 2.0 / 9.0 * t10012 - 2.0 / 9.0 * t1901 * t10015 + t446 * t10020;
    (t10008, t10009, t10014, t10015, t10018, t10020, t10022)
}
