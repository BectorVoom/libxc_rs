//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 979/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk979(t30937: f64, t8597: f64, t8602: f64, t1165: f64, t4718: f64, t7351: f64, t7426: f64, t30543: f64, t8469: f64, t4521: f64, t2268: f64, t30797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34620 = t30937 * t8597;
    let t34621 = 0.18868855373762491241e-2_f64 * t34620;
    let t34622 = t30937 * t8602;
    let t34623 = 0.37737710747524982482e-2_f64 * t34622;
    let t34626 = t7426 * t1165 * t7351 * t4718;
    let t34627 = 0.94344276868812456204e-3_f64 * t34626;
    let t34632 = t30543 * t8469;
    let t34633 = 0.18868855373762491241e-1_f64 * t34632;
    let t34636 = t7426 * t1165 * t7351 * t4521;
    let t34638 = t30797 * t2268;
    (t34621, t34623, t34627, t34633, t34636, t34638)
}
