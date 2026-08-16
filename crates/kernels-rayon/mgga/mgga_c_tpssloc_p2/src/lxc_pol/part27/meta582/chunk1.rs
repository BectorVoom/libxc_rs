//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2034/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2034(t1338: f64, t22870: f64, t22886: f64, t22892: f64, t22893: f64, t22751: f64, t22887: f64, t268: f64, t547: f64, t6559: f64) -> (f64, f64, f64, f64) {
    let t81199 = t1338 * t22870;
    let t81216 = t22892 * t22893 * t22886;
    let t81218 = t22751 * t22887;
    let t81228 = t6559 * t547 * t268;
    (t81199, t81216, t81218, t81228)
}
