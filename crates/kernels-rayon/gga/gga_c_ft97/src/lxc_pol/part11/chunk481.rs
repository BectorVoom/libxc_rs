//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 481/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk481(t2380: f64, t2697: f64, t274: f64, t2417: f64, t801: f64, t231: f64, t123: f64, t194: f64, t805: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2698 = t2697 * t2380;
    let t2699 = t2698 * t274;
    let t2701 = t801 * t2417;
    let t2702 = t2701 * t274;
    let t2704 = t2380 * t274;
    let t2705 = t231 * t2704;
    let t2710 = t123 / t805 / t194;
    (t2698, t2699, t2701, t2702, t2704, t2705, t2710)
}
