//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1035/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1035(t10980: f64, t10986: f64, t11004: f64, t11010: f64, t11015: f64, t11020: f64, t11056: f64, t11059: f64, t11062: f64, t11065: f64, t11068: f64, t11169: f64, t11181: f64, t11188: f64, t11205: f64, t8605: f64, t8607: f64, t8616: f64, t8618: f64, t8627: f64, t8629: f64, t8631: f64) -> f64 {
    let t11207 = 0.10064166666666666667e0_f64 * t8605 + 0.67094444444444444447e-1_f64 * t8607 - 0.26837777777777777778e0_f64 * t8616 - 0.20128333333333333334e0_f64 * t8618 - 0.18396666666666666667e0_f64 * t8627 + 0.5519e-1_f64 * t8629 + 0.18396666666666666667e-1_f64 * t8631 - 0.13418888888888888889e0_f64 * t10980 + t11169 - 0.301925e0_f64 * t10986 + t11181 - 0.5519e-1_f64 * t11056 - 0.27595e-1_f64 * t11059 - 0.36793333333333333333e-1_f64 * t11062 + 0.33114e0_f64 * t11065 + 0.16557e0_f64 * t11068 + t11188 - 0.40256666666666666667e0_f64 * t11004 - 0.33547222222222222222e0_f64 * t11010 + 0.12077e1_f64 * t11015 - 0.40256666666666666666e0_f64 * t11020 + t11205;
    t11207
}
