//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1759/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1759(t2684: f64, t4295: f64, t13171: f64, t860: f64, t4265: f64, t814: f64, t829: f64, t13377: f64, t235: f64, t2679: f64, t4282: f64, t4280: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13429 = t4295 * t2684;
    let t13431 = t860 * t13171;
    let t13433 = t814 * t4265;
    let t13434 = t13433 * t829;
    let t13448 = t235 * t13377;
    let t13450 = t4282 * t2679;
    let t13453 = t808 * t4280;
    (t13429, t13431, t13433, t13434, t13448, t13450, t13453)
}
