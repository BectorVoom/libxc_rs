//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 948/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk948(t288: f64, t4027: f64, t75: f64, t5042: f64, t682: f64, t1381: f64, t2955: f64, t224: f64, t4064: f64, t229: f64, t2974: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14999 = t4027 * t75 * t288;
    let t15003 = t5042 * t682;
    let t15005 = t1381 * t2955;
    let t15008 = t224 * t4064;
    let t15010 = t229 * t4064;
    let t15016 = t2974 * t484;
    (t14999, t15003, t15005, t15008, t15010, t15016)
}
