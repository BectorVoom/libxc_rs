//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1373/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1373(t77174: f64, t77189: f64, t77204: f64, t77218: f64, t942: f64, t951: f64, t959: f64, t13520: f64, t21253: f64, t10661: f64, t76644: f64, t913: f64) -> (f64, f64, f64, f64) {
    let t77220 = t77174 + t77189 + t77204 + t77218;
    let t77224 = 0.5848223622634646207e0_f64 * t959 * t942 * t77220 * t951;
    let t77226 = 24.0_f64 * t13520 * t21253;
    let t77229 = 24.0_f64 * t10661 * t76644 * t913;
    (t77220, t77224, t77226, t77229)
}
