//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1140/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1140(t11522: f64, t15805: f64, t9799: f64, t34104: f64, t34108: f64, t34111: f64, t34114: f64, t34117: f64, t34119: f64, t34121: f64, t34125: f64, t34127: f64, t34132: f64) -> f64 {
    let t34135 = t15805 * t11522 * t9799;
    let t34137 = 0.2318836277704281739e-4_f64 * t34104 + 0.56360603971979070047e-7_f64 * t34108 + 0.34752370105806885418e-3_f64 * t34111 - 0.24581606547037760418e-7_f64 * t34114 + 0.12290803273518880209e-8_f64 * t34117 - 0.35170937063461460536e-8_f64 * t34119 - 0.35170937063461460536e-8_f64 * t34121 + 0.4797801045921060808e-7_f64 * t34125 + 0.17089546493091976008e-5_f64 * t34127 - 0.12290803273518880209e-8_f64 * t34132 + 0.12650553385416666667e-5_f64 * t34135;
    t34137
}
