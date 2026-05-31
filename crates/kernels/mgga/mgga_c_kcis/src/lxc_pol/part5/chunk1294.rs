//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1294/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1294<F: Float>(t21400: F, t469: F, t21196: F, t21199: F, t21201: F, t21203: F, t21206: F, t21209: F, t21212: F, t21215: F, t21218: F, t21221: F, t21224: F) -> (F, F) {
    let t21402 = F::cast_from(0.62182e-1_f64) * t21400 * t469;
    let t21424 = F::cast_from(0.10064166666666666667e0_f64) * t21196 - F::cast_from(0.82785e-1_f64) * t21199 - F::cast_from(0.11038e0_f64) * t21201 + F::cast_from(0.5519e-1_f64) * t21203 - F::cast_from(0.24154e1_f64) * t21206 - F::cast_from(0.20128333333333333333e0_f64) * t21209 + F::cast_from(0.60385e0_f64) * t21212 + F::cast_from(0.11038e0_f64) * t21215 - F::cast_from(0.49671e0_f64) * t21218 - F::cast_from(0.66228e0_f64) * t21221 + F::cast_from(0.16557e0_f64) * t21224;
    (t21402, t21424)
}
