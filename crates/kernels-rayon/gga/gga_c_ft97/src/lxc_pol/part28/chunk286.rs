//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 286/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk286(t1787: f64, t3009: f64, t2: f64, t942: f64, t1587: f64, t432: f64, t24: f64, t3103: f64, t469: f64, t1773: f64, t1776: f64, t1778: f64, t3125: f64, t3128: f64, t3131: f64, t3135: f64, t3139: f64, t3141: f64, t3144: f64, t462: f64, t92: f64) -> f64 {
    let t3146 = t1787 * t3009;
    let t3149 = t2 * t942;
    let t3151 = t1587 * t3149 * t432;
    let t3155 = t24 * t469 * t3103;
    let t3157 = t1773 + t1776 / 9.0_f64 + t1778 / 3.0_f64 + t3125 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t462 * t3128 + t462 * t3131 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t3135 - 2.0_f64 / 3.0_f64 * t3139 * t3141 + t3144 / 3.0_f64 + t462 * t3146 / 3.0_f64 + 2.0_f64 * t462 * t3151 - t92 * t3155;
    t3157
}
