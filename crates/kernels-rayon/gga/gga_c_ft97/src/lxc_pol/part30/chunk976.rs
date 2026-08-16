//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 976/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk976(t143365: f64, t1882: f64, t33980: f64, t33953: f64, t668: f64, t25409: f64, t7581: f64, t143263: f64, t143273: f64, t143332: f64, t143335: f64, t34281: f64, t6210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t143366 = 2.0_f64 / 27.0_f64 * t143365;
    let t143371 = t1882 * t33980;
    let t143373 = t33953 * t668;
    let t143432 = t7581 * t25409;
    let t143497 = 8.0_f64 / 9.0_f64 * t143263;
    let t143500 = 10.0_f64 / 9.0_f64 * t143273;
    let t143518 = 4.0_f64 / 9.0_f64 * t143332;
    let t143519 = 4.0_f64 / 9.0_f64 * t143335;
    let t143528 = 2.0_f64 / 9.0_f64 * t143365;
    let t143538 = t6210 * t34281;
    (t143366, t143371, t143373, t143432, t143497, t143500, t143518, t143519, t143528, t143538)
}
