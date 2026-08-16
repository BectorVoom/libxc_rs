//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 840/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk840(t182: f64, t5151: f64, t172: f64, t1787: f64, t763: f64, t67: f64, t758: f64, t193: f64, t533: f64, t1845: f64, t3701: f64, t3692: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5153 = 0.19751673498613801407e-1_f64 * t5151 * t182;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5156 = 0.5848223622634646207e0_f64 * t5155;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5159 = 0.18311447306006545054e-3_f64 * t5158;
    let t5160 = t193 * t533;
    let t5161 = t1845 * t3701;
    let t5164 = 0.5848223622634646207e0_f64 * t3692;
    (t5153, t5154, t5156, t5157, t5159, t5160, t5161, t5164)
}
