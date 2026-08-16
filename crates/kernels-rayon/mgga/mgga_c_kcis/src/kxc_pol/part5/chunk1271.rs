//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1271/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1271(t16906: f64, t21134: f64, t25: f64, t7081: f64, t493: f64, t1930: f64, t5718: f64, t1368: f64, t1382: f64, t16850: f64, t21103: f64, t21107: f64, t21111: f64, t21117: f64, t21121: f64, t21126: f64, t21131: f64, t5691: f64, t5723: f64, t5734: f64, t7054: f64) -> f64 {
    let t21135 = t16906 * t21134;
    let t21138 = t25 * t7081;
    let t21139 = t493 * t21138;
    let t21141 = t1930 * t5718;
    let t21148 = t1368 * t21103 / 144.0_f64 + t1368 * t21107 / 48.0_f64 + t1368 * t21111 / 36.0_f64 + t5691 * t5723 / 54.0_f64 - t1368 * t21117 / 144.0_f64 - t1368 * t21121 / 216.0_f64 - t1368 * t21126 / 36.0_f64 + 7.0_f64 / 648.0_f64 * t1368 * t21131 - t1368 * t21135 / 54.0_f64 + t21139 / 144.0_f64 + t21141 / 54.0_f64 + t1930 * t5734 / 18.0_f64 - 11.0_f64 / 108.0_f64 * t7054 * t1382 - t16850 / 216.0_f64;
    t21148
}
