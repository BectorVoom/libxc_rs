//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1023/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1023(t3866: f64, t5310: f64, t3799: f64, t5289: f64, t2371: f64, t5154: f64, t5151: f64, t67: f64, t758: f64, t12365: f64, t1827: f64, t12300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16147 = 35.0_f64 / 576.0_f64 * t3866 * t5310;
    let t16159 = 7.0_f64 / 2304.0_f64 * t3799 * t5289;
    let t16164 = t5154 * t2371;
    let t16169 = t5151 * t67;
    let t16171 = 0.36622894612013090108e-3_f64 * t16169 * t758;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0_f64 / 2304.0_f64 * t12300 * t1827;
    (t16147, t16159, t16164, t16171, t16211, t16214)
}
