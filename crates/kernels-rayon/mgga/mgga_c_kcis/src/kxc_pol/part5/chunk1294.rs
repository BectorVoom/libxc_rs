//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1294/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1294(t21400: f64, t469: f64, t21196: f64, t21199: f64, t21201: f64, t21203: f64, t21206: f64, t21209: f64, t21212: f64, t21215: f64, t21218: f64, t21221: f64, t21224: f64) -> (f64, f64) {
    let t21402 = 0.62182e-1_f64 * t21400 * t469;
    let t21424 = 0.10064166666666666667e0_f64 * t21196 - 0.82785e-1_f64 * t21199 - 0.11038e0_f64 * t21201 + 0.5519e-1_f64 * t21203 - 0.24154e1_f64 * t21206 - 0.20128333333333333333e0_f64 * t21209 + 0.60385e0_f64 * t21212 + 0.11038e0_f64 * t21215 - 0.49671e0_f64 * t21218 - 0.66228e0_f64 * t21221 + 0.16557e0_f64 * t21224;
    (t21402, t21424)
}
