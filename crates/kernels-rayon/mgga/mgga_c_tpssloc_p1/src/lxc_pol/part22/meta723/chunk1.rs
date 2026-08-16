//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2369/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2369(t16558: f64, t4342: f64, t136: f64, t908: f64, t17156: f64, t3966: f64, t2826: f64, t13527: f64, t5398: f64, t4337: f64, t20234: f64, t41666: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t68458 = t4342 * t16558;
    let t68460 = t136 * t908 * t68458;
    let t68462 = t17156 * t3966;
    let t68464 = t136 * t2826 * t68462;
    let t68466 = t13527 * t5398;
    let t68468 = t136 * t2826 * t68466;
    let t68470 = t4337 * t16558;
    let t68472 = t136 * t2826 * t68470;
    let t68477 = t41666 * t20234 * t607;
    (t68458, t68460, t68462, t68464, t68466, t68468, t68470, t68472, t68477)
}
