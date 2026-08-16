//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3255/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3255(t6016: f64, t836: f64, t10811: f64, t18482: f64, t5977: f64, t14749: f64, t14785: f64, t14791: f64, t1559: f64, t2745: f64, t2749: f64, t50518: f64, t50522: f64, t50524: f64, t50526: f64, t50529: f64, t50531: f64, t50540: f64) -> (f64, f64, f64) {
    let t61749 = t6016 * t836;
    let t61754 = t10811 * t18482;
    let t61756 = t5977 * t836;
    let t61772 = 0.17149607247227894789e-2_f64 * t2745 * t14791 * t61749 * t2749 + 0.80031500487063509016e-1_f64 * t61754 + 0.17149607247227894789e-2_f64 * t2745 * t14791 * t61756 * t2749 - 0.17149607247227894789e-1_f64 * t2745 * t14785 * t1559 * t14749 + 0.11433071498151929859e-3_f64 * t50518 - 0.85748036236139473944e-4_f64 * t50522 + 0.45351183609335988442e0_f64 * t50524 - 0.80031500487063509016e-1_f64 * t50526 - 0.40015750243531754508e-1_f64 * t50529 - 0.10841600599314203355e-1_f64 * t50531 - 0.30492001685571196935e-2_f64 * t50540;
    (t61749, t61756, t61772)
}
