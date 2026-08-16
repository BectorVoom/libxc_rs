//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 715/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk715(t13628: f64, t701: f64, t2436: f64, t3799: f64, t1103: f64, t228: f64, t231: f64, t625: f64, t1123: f64, t626: f64, t1095: f64, t694: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13629 = t701 * t13628;
    let t13635 = t3799 * t2436;
    let t13636 = 0.1134997482304526749e-1_f64 * t13635;
    let t13643 = t228 * t1103 * t625 * t231;
    let t13647 = t626 * t1123;
    let t13648 = t701 * t13647;
    let t13654 = t694 * t1095;
    (t13629, t13635, t13636, t13643, t13648, t13654)
}
