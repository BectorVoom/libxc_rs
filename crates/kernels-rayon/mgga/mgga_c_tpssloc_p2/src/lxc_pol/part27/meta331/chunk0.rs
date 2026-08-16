//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1410/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1410(t1285: f64, t2221: f64, t1287: f64, t522: f64, t9216: f64, t9218: f64, t1294: f64, t9713: f64, t25: f64, t526: f64, t28: f64, t528: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12050 = t2221 * t1285;
    let t12052 = t2221 * t1287;
    let t12054 = t9216 * t522;
    let t12057 = 120.0_f64 * t9218 * t522;
    let t12059 = 0.5848223622634646207e0_f64 * t1294 * t9713;
    let t12061 = 1.0_f64 / t526 / t25;
    let t12072 = 1.0_f64 / t528 / t28;
    (t12050, t12052, t12054, t12057, t12059, t12061, t12072)
}
