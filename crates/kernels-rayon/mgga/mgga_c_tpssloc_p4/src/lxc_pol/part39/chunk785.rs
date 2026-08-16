//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 785/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk785(t4189: f64, t4264: f64, t218: f64, t1520: f64, t225: f64, t1527: f64, t865: f64, t2718: f64, t2627: f64, t68: f64, t226: f64, t1509: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4265 = t4189 + t4264;
    let t4266 = t218 * t4265;
    let t4268 = t1520 * t225;
    let t4272 = t1527 * t865;
    let t4273 = t2718 * t4272;
    let t4280 = t68 * t2627;
    let t4281 = t226 * t4280;
    let t4282 = t252 * t1509;
    (t4265, t4266, t4268, t4273, t4280, t4281, t4282)
}
