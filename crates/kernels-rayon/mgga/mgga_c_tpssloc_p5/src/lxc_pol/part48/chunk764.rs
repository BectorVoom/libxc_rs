//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 764/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk764(t24046: f64, t24062: f64, t539: f64, t22645: f64, t225: f64, t7192: f64, t2091: f64, t3887: f64, t3911: f64, t12021: f64, t3888: f64, t7179: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24063 = t24046 + t24062;
    let t24064 = t539 * t24063;
    let t24071 = 0.16449340668482264365e-1_f64 * t22645;
    let t24082 = t7192 * t225;
    let t24088 = t3887 * t2091 * t3911;
    let t24092 = t12021 * t2091 * t3888;
    let t24095 = t7179 * t225;
    (t24063, t24064, t24071, t24082, t24088, t24092, t24095)
}
