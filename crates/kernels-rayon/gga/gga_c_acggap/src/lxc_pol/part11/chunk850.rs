//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 850/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk850(t7927: f64, t880: f64, t7884: f64, t7911: f64, t7887: f64, t7930: f64, t862: f64, t309: f64, t871: f64, t620: f64, t1210: f64, t618: f64) -> (f64, f64, f64, f64, f64) {
    let t29973 = 0.19756347548806534796e1_f64 * t7927 * t880;
    let t29976 = t7884 * t7911;
    let t29977 = t29976 * t7887;
    let t29979 = t862 * t7930;
    let t29980 = t871 * t309;
    let t29982 = t29979 * t620 * t29980;
    let t29984 = t1210 * t618;
    (t29973, t29977, t29979, t29982, t29984)
}
