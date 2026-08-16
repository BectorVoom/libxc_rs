//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1239/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1239(t10769: f64, t828: f64, t1544: f64, t836: f64, t2749: f64, t2746: f64, t14494: f64, t775: f64, t14586: f64, t10693: f64, t10706: f64, t10711: f64, t10713: f64, t10717: f64, t10719: f64, t10723: f64, t10730: f64, t10734: f64, t10742: f64, t2745: f64, t4362: f64) -> f64 {
    let t14785 = t10769 * t828;
    let t14786 = t1544 * t836;
    let t14787 = t14786 * t2749;
    let t14788 = t14785 * t14787;
    let t14791 = t2746 * t828;
    let t14792 = t14494 * t2749;
    let t14793 = t14791 * t14792;
    let t14802 = t775 * t836;
    let t14803 = t14586 * t14802;
    let t14804 = t14791 * t14803;
    let t14811 = -0.85748036236139473944e-2_f64 * t2745 * t14788 + 0.17149607247227894789e-2_f64 * t2745 * t14793 - 0.20007875121765877254e-1_f64 * t10693 + 0.25410001404642664112e-3_f64 * t10706 + 0.71456696863449561619e-5_f64 * t10711 + 0.40015750243531754508e-2_f64 * t10713 + 0.10841600599314203354e-2_f64 * t10717 - 0.15244095330869239812e-3_f64 * t10719 - 0.34299214494455789578e-2_f64 * t4362 * t14804 - 0.45351183609335988442e-1_f64 * t10723 - 0.14291339372689912324e-4_f64 * t10730 + 0.71456696863449561619e-5_f64 * t10734 - 0.50820002809285328224e-4_f64 * t10742;
    t14811
}
