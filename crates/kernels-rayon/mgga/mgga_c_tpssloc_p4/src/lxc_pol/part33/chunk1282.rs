//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1282/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1282(t252: f64, t5544: f64, t23110: f64, t28337: f64, t81651: f64, t28423: f64, t6579: f64, t28427: f64, t28419: f64, t22893: f64, t28341: f64, t81640: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98422 = t252 * t5544;
    let t98446 = t81651 * t23110 * t28337;
    let t98488 = t6579 * t28423;
    let t98490 = t6579 * t28427;
    let t98505 = t6579 * t28419;
    let t98516 = t81640 * t22893 * t28341;
    (t98422, t98446, t98488, t98490, t98505, t98516)
}
