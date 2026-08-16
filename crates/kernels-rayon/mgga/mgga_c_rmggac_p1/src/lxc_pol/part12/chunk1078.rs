//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1078/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1078(t36935: f64, t9082: f64, t202: f64, t461: f64, t5527: f64, t674: f64, t678: f64, t2185: f64, t9086: f64, t16043: f64, t9051: f64, t9055: f64) -> (f64, f64, f64, f64, f64) {
    let t42250 = t36935 * t9082;
    let t42255 = t5527 * t202 * t461 * t674 * t678;
    let t42258 = t9086 * t2185 * t678;
    let t42259 = 0.19863479950205658386e-4_f64 * t42258;
    let t42260 = t16043 * t9051;
    let t42262 = t16043 * t9055;
    (t42250, t42255, t42259, t42260, t42262)
}
