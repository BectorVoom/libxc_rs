//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 378/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk378(t1382: f64, t1384: f64, t1386: f64, t1391: f64, t1358: f64, t1710: f64, t1712: f64, t684: f64, t693: f64, t805: f64, t905: f64, t659: f64, t708: f64, t711: f64, t714: f64, t717: f64, t753: f64, t757: f64, t764: f64, t774: f64, t782: f64, t809: f64, t914: f64) -> (f64, f64) {
    let t1820 = 0.11696447245269292414e1_f64 * t1382;
    let t1821 = 8.0_f64 * t1384;
    let t1822 = 8.0_f64 * t1386;
    let t1823 = 2.0_f64 * t1391;
    let t1824 = 0.36622894612013090108e-3_f64 * t1358;
    let t1825 = t1712 + t1710 - t1820 - t1821 - t1822 + t1823 - t1824 - t684 - t693 + t805 - t905;
    let t1826 = -t708 - t764 + t711 + t714 + t717 - t753 + t774 + t782 + t659 + t809 + t914 - t757;
    (t1825, t1826)
}
