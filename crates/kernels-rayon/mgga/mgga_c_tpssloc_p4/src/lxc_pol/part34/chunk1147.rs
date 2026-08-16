//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1147/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1147(t23110: f64, t23185: f64, t28422: f64, t23168: f64, t28346: f64, t28338: f64, t81591: f64, t252: f64, t5544: f64, t28337: f64, t81651: f64, t28423: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98399 = t23185 * t23110 * t28422;
    let t98416 = t23168 * t28346;
    let t98420 = t81591 * t28338;
    let t98422 = t252 * t5544;
    let t98446 = t81651 * t23110 * t28337;
    let t98488 = t6579 * t28423;
    (t98399, t98416, t98420, t98422, t98446, t98488)
}
