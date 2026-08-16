//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1754/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1754(t2496: f64, t9551: f64, t4038: f64, t9372: f64, t1317: f64, t9428: f64, t3853: f64, t3857: f64, t40076: f64, t40079: f64, t47131: f64, t47134: f64, t47136: f64, t47138: f64, t47140: f64, t47142: f64, t47144: f64) -> (f64, f64, f64, f64, f64) {
    let t47145 = t9551 * t2496;
    let t47146 = 0.10389515463408878255e3_f64 * t47145;
    let t47147 = t4038 * t9372;
    let t47148 = 0.4101607543286562663e4_f64 * t47147;
    let t47149 = t1317 * t9428;
    let t47150 = 48.0_f64 * t47149;
    let t47152 = 120.0_f64 * t3857 * t3853;
    let t47153 = t47131 + t47134 - t47136 - t47138 - t47140 + t47142 + t47144 + t40076 - t40079 - t47146 - t47148 + t47150 + t47152;
    (t47146, t47148, t47150, t47152, t47153)
}
