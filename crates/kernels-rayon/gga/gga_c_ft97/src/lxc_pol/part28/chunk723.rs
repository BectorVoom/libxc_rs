//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 723/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk723(t108: f64, t7165: f64, t379: f64, t7824: f64, t5498: f64, t7162: f64, t5493: f64, t7150: f64, t1322: f64, t1774: f64, t7151: f64, t1308: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32019 = t7165 * t108;
    let t32021 = t7824 * t32019 * t379;
    let t32025 = t7162 * t5498 / 18.0_f64;
    let t32026 = t5493 * t7150;
    let t32029 = t1774 * t1322;
    let t32031 = t7151 * t32029 / 18.0_f64;
    let t32032 = t1308 * t379;
    (t32019, t32021, t32025, t32026, t32029, t32031, t32032)
}
