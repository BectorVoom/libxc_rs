//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1294/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1294(t2165: f64, t61057: f64, t17960: f64, t2367: f64, t17942: f64, t223: f64, t764: f64, t17946: f64, t2153: f64, t238: f64, t5543: f64, t2149: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t61058 = t61057 * t2165;
    let t61060 = t17960 * t2367;
    let t61062 = t17942 * t223;
    let t61063 = t61062 * t764;
    let t61065 = t17946 * t2153;
    let t61072 = t5543 * t238;
    let t61073 = t61072 * t2149;
    (t61058, t61060, t61062, t61063, t61065, t61073)
}
