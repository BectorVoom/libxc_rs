//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 998/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk998<F: Float>(t292: F, t43789: F, t1771: F, t2783: F, t3051: F, t854: F, t10603: F, t13682: F, t13688: F, t15042: F, t15047: F, t192: F, t19714: F, t2771: F, t2781: F, t43386: F, t43397: F, t43414: F, t43513: F, t43553: F, t43563: F, t43568: F, t43574: F, t43578: F, t462: F, t824: F, t852: F, t92: F) -> (F, F) {
    let t293 = 0.1e-59 < t292;
    let t43790 = piecewise3(t293, t43789, 0.0);
    let t43794 = t1771 * t2783;
    let t43796 = t3051 * t854;
    let t43798 = 8.0 / 3.0 * t13682 * t15042 * t43553 + 4.0 / 3.0 * t462 * t2771 * t43414 + 2.0 * t462 * t2771 * t43397 - 4.0 / 3.0 * t43563 - 8.0 * t13688 * t15047 * t43553 - 8.0 * t13688 * t19714 * t43568 * t824 + t43574 + 8.0 * t462 * t10603 * t43386 + 8.0 / 3.0 * t43578 + 6.0 * t92 * t192 * t2781 * t43513 - t92 * t192 * t852 * t43790 + 16.0 / 3.0 * t43794 + 112.0 / 27.0 * t43796;
    (t43790, t43798)
}
