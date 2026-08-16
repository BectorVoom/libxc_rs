//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1677/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1677(t3552: f64, t3781: f64, t1204: f64, t13147: f64, t13141: f64, t3596: f64, t42859: f64, t460: f64, t3603: f64, t43351: f64, t1214: f64, t17703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45764 = t3552 * t3781;
    let t45769 = t1204 * t13147;
    let t45779 = t1204 * t13141;
    let t45785 = t42859 * t3596;
    let t45786 = t460 * t45785;
    let t45787 = t43351 * t3603;
    let t45796 = t17703 * t1214;
    (t45764, t45769, t45779, t45786, t45787, t45796)
}
