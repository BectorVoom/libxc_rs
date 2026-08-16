//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1315/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1315(t21262: f64, t750: f64, t17930: f64, t821: f64, t19817: f64, t14245: f64, t19671: f64, t1398: f64, t3610: f64, t1288: f64, t3683: f64, t823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t69799 = t21262 * t750;
    let t69800 = t17930 * t69799;
    let t69803 = t21262 * t821;
    let t69804 = t19817 * t69803;
    let t69807 = t19671 * t14245;
    let t69810 = t3610 * t1398;
    let t69811 = t17930 * t69810;
    let t69817 = t823 * t1288 * t3683;
    (t69799, t69800, t69803, t69804, t69807, t69810, t69811, t69817)
}
