//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1890/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1890(t12971: f64, t28: f64, t1081: f64, t4119: f64, t13191: f64, t25891: f64, t25927: f64, t57921: f64, t13471: f64, t1484: f64, t3231: f64, t86781: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89888 = t28 * t12971;
    let t89892 = t1081 * t4119;
    let t89896 = t25891 * t13191;
    let t89904 = t25927 * t57921;
    let t89907 = t28 * t13471;
    let t89911 = t3231 * t1484;
    let t89917 = t25927 * t86781;
    (t89888, t89892, t89896, t89904, t89907, t89911, t89917)
}
