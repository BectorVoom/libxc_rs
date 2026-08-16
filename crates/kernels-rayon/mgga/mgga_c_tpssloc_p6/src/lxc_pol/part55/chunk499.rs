//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 499/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk499(t964: f64, t995: f64, t1050: f64, t225: f64, t1053: f64, t386: f64, t68: f64, t1057: f64, t3112: f64, t3032: f64, t3127: f64, t3031: f64) -> (f64, f64, f64, f64, f64) {
    let t3163 = t964 * t995;
    let t3169 = t1050 * t225;
    let t3173 = 1.0_f64 / t1053 / t386;
    let t3174 = t68 * t3173;
    let t3180 = t3112 * t1057;
    let t3185 = t3032 * t3127;
    let t3186 = t3031 * t3185;
    (t3163, t3169, t3174, t3180, t3186)
}
