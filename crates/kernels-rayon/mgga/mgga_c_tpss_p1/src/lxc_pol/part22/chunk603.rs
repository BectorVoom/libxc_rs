//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 603/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk603(t2703: f64, t345: f64, t947: f64, t242: f64, t196: f64, t934: f64, param_beta: f64) -> (f64, f64, f64) {
    let t2704 = t2703 * t345;
    let t2705 = t947 * t2704;
    let t2706 = t242 * t2705;
    let t2710 = 1.0_f64 / t934 / t196;
    let t2711 = param_beta * t2710;
    (t2704, t2706, t2711)
}
