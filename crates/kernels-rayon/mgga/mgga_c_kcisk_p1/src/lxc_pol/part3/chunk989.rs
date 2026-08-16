//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 989/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk989(t13369: f64, t6317: f64, t4203: f64, t1492: f64, t4240: f64, t486: f64, t13777: f64, t4143: f64, t487: f64, t13288: f64, t499: f64, t498: f64) -> (f64, f64, f64, f64) {
    let t14567 = t6317 * t13369;
    let t14568 = t4203 * t14567;
    let t14570 = t1492 * t4240;
    let t14571 = t486 * t14570;
    let t14573 = t4143 * t13777;
    let t14574 = t487 * t14573;
    let t14575 = t486 * t14574;
    let t14577 = t499 * t13288;
    let t14578 = t498 * t14577;
    (t14568, t14571, t14575, t14578)
}
