//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1104/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1104(t11938: f64, t325: f64, t33712: f64, t11803: f64, t11804: f64, t19196: f64, t11775: f64, t29207: f64, t147: f64, t19: f64, t512: f64, t1038: f64, t2619: f64, t297: f64, t7371: f64) -> (f64, f64, f64, f64, f64) {
    let t33714 = t325 * t33712 * t11938;
    let t33717 = t11803 * t11804 * t19196;
    let t33719 = t11775 * t29207;
    let t33722 = t512 * t19 * t147;
    let t33726 = t2619 * t33722 * t1038 * t297 * t7371;
    (t33714, t33717, t33719, t33722, t33726)
}
