//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2100/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2100(t10046: f64, t814: f64, t225: f64, t9520: f64, t10647: f64, t892: f64, t2784: f64, t2841: f64, t22715: f64, t268: f64, t271: f64) -> (f64, f64, f64, f64, f64) {
    let t41520 = t814 * t10046;
    let t41554 = t9520 * t225;
    let t41618 = t10647 * t892;
    let t41623 = t2784 * t2841;
    let t41654 = t268 * t22715 * t271;
    (t41520, t41554, t41618, t41623, t41654)
}
