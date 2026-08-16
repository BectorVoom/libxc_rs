//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1127/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1127(t16416: f64, t654: f64, t16412: f64, t9686: f64, t16398: f64, t2030: f64, t16323: f64, t2024: f64, t16370: f64, t16443: f64, t669: f64, t16493: f64, t2144: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48555 = t654 * t16416;
    let t48559 = t9686 * t16412;
    let t48571 = t2030 * t16398;
    let t48577 = t16323 * t2024;
    let t48590 = t16370 * t2024;
    let t48629 = t16443 * t669;
    let t48744 = t2144 * t16493;
    (t48555, t48559, t48571, t48577, t48590, t48629, t48744)
}
