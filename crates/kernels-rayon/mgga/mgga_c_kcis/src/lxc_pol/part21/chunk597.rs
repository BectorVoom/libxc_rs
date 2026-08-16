//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 597/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk597(t2970: f64, t4567: f64, t26: f64, t4581: f64, t945: f64, t22: f64, t2470: f64) -> (f64, f64, f64, f64, f64) {
    let t4708 = t2970 * t4567;
    let t4709 = t26 * t4708;
    let t4711 = t945 * t4581;
    let t4712 = t26 * t4711;
    let t4714 = t22 * t2470;
    (t4708, t4709, t4711, t4712, t4714)
}
