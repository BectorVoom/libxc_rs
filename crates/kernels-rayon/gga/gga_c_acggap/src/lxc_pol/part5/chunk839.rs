//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 839/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk839(t218: f64, t2693: f64, t760: f64, t715: f64, t902: f64, t2801: f64, t2803: f64, t761: f64, t771: f64, t229: f64, t2825: f64, t2809: f64, t780: f64) -> (f64, f64, f64, f64, f64) {
    let t11756 = 8.0_f64 * t760 * t2693 * t218;
    let t11762 = t715 * t902;
    let t11770 = 0.3103560775156404018e4_f64 * t2801 * t761 * t2803 * t771;
    let t11772 = 16.0_f64 * t229 * t2825;
    let t11775 = 0.57895126195293126241e3_f64 * t2809 * t780 * t771;
    (t11756, t11762, t11770, t11772, t11775)
}
