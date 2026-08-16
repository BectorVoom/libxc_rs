//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 997/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk997(t5309: f64, t875: f64, t10697: f64, t296: f64, t1248: f64, t4299: f64, t2843: f64, t5424: f64, t824: f64, t840: f64, t2862: f64, t5225: f64, t882: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19430 = t5309 * t875;
    let t19431 = t10697 * t19430;
    let t19432 = t296 * t19431;
    let t19435 = t1248 * t4299;
    let t19436 = t2843 * t19435;
    let t19437 = t296 * t19436;
    let t19442 = t840 * t5424 * t824;
    let t19446 = t2862 * t882 * t5225;
    (t19431, t19432, t19436, t19437, t19442, t19446)
}
