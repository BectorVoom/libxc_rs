//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 517/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk517(t2435: f64, t2439: f64, t2502: f64, t2504: f64, t2509: f64, t2511: f64) -> f64 {
    let t2576 = -0.42198333333333333333e0_f64 * t2502 + 0.84396666666666666666e0_f64 * t2504 + 0.39862222222222222223e0_f64 * t2435 + 0.68258333333333333333e-1_f64 * t2509 + 0.13651666666666666667e0_f64 * t2511 + 0.13692777777777777778e0_f64 * t2439;
    t2576
}
