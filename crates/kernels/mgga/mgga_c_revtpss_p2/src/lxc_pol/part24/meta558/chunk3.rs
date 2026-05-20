//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1672/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1672<F: Float>(t11387: F, t41588: F, t88031: F, t41592: F, t77499: F, t77505: F, t77507: F, t77509: F, t77663: F, t77667: F, t88089: F, t88097: F, t88144: F, t88147: F, t88150: F, t88161: F, t88164: F) -> (F, F) {
    let t88368 = F::cast_from(0.62071215503128080361e4_f64) * t41588 * t88031 * t11387;
    let t88382 = -F::cast_from(0.85199506172839506175e-1_f64) * t88144 - F::cast_from(0.82156666666666666667e-1_f64) * t88147 + F::cast_from(0.43816888888888888889e0_f64) * t88150 - F::cast_from(0.43816888888888888888e0_f64) * t77663 + F::cast_from(0.97370864197530864196e-1_f64) * t77667 - F::new(0.107628e2) * t88089 + F::cast_from(0.23917333333333333333e1_f64) * t88097 + t41592 + F::cast_from(0.44291358024691358024e0_f64) * t77499 + F::cast_from(0.39862222222222222223e0_f64) * t77505 - F::cast_from(0.15944888888888888889e1_f64) * t77507 + F::cast_from(0.23917333333333333333e1_f64) * t77509 - F::cast_from(0.98587999999999999998e0_f64) * t88161 - F::cast_from(0.82156666666666666668e-1_f64) * t88164;
    (t88368, t88382)
}
