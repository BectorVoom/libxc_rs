//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1050/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1050(t1981: f64, t41799: f64, t41800: f64, t676: f64, t236: f64, t498: f64, t3134: f64, t8512: f64, t1982: f64, t7428: f64, t8511: f64, t16156: f64, t9198: f64) -> (f64, f64, f64, f64) {
    let t41803 = t41799 * t1981 * t676 * t41800;
    let t41805 = t236 * t498;
    let t41808 = t8512 * t1981 * t3134 * t41805;
    let t41811 = t8511 * t7428 * t1982;
    let t41812 = 0.19863479950205658386e-4_f64 * t41811;
    let t41813 = t16156 * t9198;
    (t41803, t41808, t41812, t41813)
}
