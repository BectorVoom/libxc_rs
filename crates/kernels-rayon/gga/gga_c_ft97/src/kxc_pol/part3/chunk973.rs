//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 973/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk973(t280: f64, t39: f64, t2035: f64, t1109: f64, t1208: f64, t820: f64, t14722: f64, t1196: f64, t817: f64, t800: f64, t4100: f64, t7853: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19038 = t280 * t39;
    let t19039 = t19038 * t2035;
    let t19043 = t1109 * t1208;
    let t19044 = t19043 * t820;
    let t19045 = t14722 * t19044;
    let t19048 = t817 * t1196;
    let t19049 = t800 * t19048;
    let t19050 = t7853 * t4100;
    (t19039, t19043, t19045, t19048, t19049, t19050)
}
