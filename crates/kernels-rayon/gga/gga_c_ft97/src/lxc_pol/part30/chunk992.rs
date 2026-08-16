//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 992/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk992(t2371: f64, t35516: f64, t1403: f64, t35549: f64, t681: f64, t35752: f64, t35546: f64, t761: f64, t766: f64, t35281: f64, t10052: f64, t140664: f64, t140684: f64, t193: f64, t24191: f64, t27956: f64, t33253: f64, t35285: f64, t35604: f64, t35779: f64, t41409: f64, t6009: f64, t6011: f64, t6062: f64, t6187: f64, t6930: f64, t6945: f64) -> (f64, f64, f64) {
    let t149920 = t2371 * t35516;
    let t149926 = t1403 * t681 * t35549;
    let t149929 = t1403 * t681 * t35752;
    let t149949 = t35546 * t761;
    let t149950 = t149949 * t766;
    let t149953 = t1403 * t681 * t35281;
    let t149959 = -t1403 * t193 * t149920 * t6009 / 3.0_f64 + t149926 / 9.0_f64 - t149929 / 18.0_f64 - t140664 / 18.0_f64 - t1403 * t193 * t33253 * t27956 / 3.0_f64 + 48.0_f64 * t41409 * t35604 * t766 - 24.0_f64 * t10052 * t6930 * t6187 - t35779 * t6011 / 3.0_f64 - t140684 / 9.0_f64 + t1403 * t193 * t6062 * t6945 / 3.0_f64 - 2.0_f64 * t149950 - t149953 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t1403 * t193 * t24191 * t35285;
    (t149920, t149950, t149959)
}
