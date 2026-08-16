//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 994/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk994(t1978: f64, t7228: f64, t8511: f64, t236: f64, t495: f64, t1981: f64, t676: f64, t498: f64, t3134: f64, t8512: f64, t1982: f64, t7428: f64) -> (f64, f64, f64) {
    let t41799 = t8511 * t7228 * t1978;
    let t41800 = t236 * t495;
    let t41803 = t41799 * t1981 * t676 * t41800;
    let t41805 = t236 * t498;
    let t41808 = t8512 * t1981 * t3134 * t41805;
    let t41811 = t8511 * t7428 * t1982;
    (t41803, t41808, t41811)
}
