//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2248/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2248(t28359: f64, t838: f64, t23069: f64, t5572: f64, t23062: f64, t28383: f64, t20986: f64, t2628: f64, t6605: f64, t828: f64, t17004: f64, t6581: f64) -> (f64, f64, f64, f64, f64) {
    let t98690 = t28359 * t838;
    let t98694 = t23069 * t5572;
    let t98696 = t23062 * t28383;
    let t98701 = t6605 * t2628 * t20986 * t828;
    let t98703 = t6581 * t17004;
    (t98690, t98694, t98696, t98701, t98703)
}
