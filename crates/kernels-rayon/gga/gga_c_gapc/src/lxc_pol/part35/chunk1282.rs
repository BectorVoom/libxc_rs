//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1282/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1282(t11675: f64, t24271: f64, t10349: f64, t11694: f64, t332: f64, t3225: f64, t10153: f64, t35751: f64, t6182: f64, t11683: f64, t11687: f64, t22442: f64) -> (f64, f64, f64, f64, f64) {
    let t35831 = t11675 * t24271;
    let t35834 = t11694 * t332 * t10349;
    let t35835 = t3225 * t35834;
    let t35838 = t10153 * t35751 * t6182;
    let t35841 = t11687 * t11683 * t22442;
    (t35831, t35834, t35835, t35838, t35841)
}
