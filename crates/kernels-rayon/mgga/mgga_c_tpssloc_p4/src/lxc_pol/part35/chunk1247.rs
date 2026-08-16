//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1247/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1247(t1519: f64, t5584: f64, t20852: f64, t252: f64, t5611: f64, t1509: f64, t5631: f64, t21064: f64, t225: f64, t22398: f64, t22334: f64, t22337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t67358 = t1519 * t5584;
    let t67392 = t252 * t20852;
    let t67405 = t1519 * t5611;
    let t68025 = t5611 * t1509;
    let t68217 = t5631 * t1509;
    let t68322 = t21064 * t225;
    let t73613 = t22398 * t225;
    let t73856 = t22334 * t225;
    let t73891 = t22337 * t225;
    (t67358, t67392, t67405, t68025, t68217, t68322, t73613, t73856, t73891)
}
