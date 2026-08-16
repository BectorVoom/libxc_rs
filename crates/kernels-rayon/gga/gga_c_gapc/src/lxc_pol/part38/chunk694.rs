//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 694/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk694(t103: f64, t2761: f64, t332: f64, t7875: f64, t7877: f64, t2627: f64, t442: f64, t919: f64, t818: f64, t1087: f64, t2232: f64, t1086: f64) -> (f64, f64, f64, f64, f64) {
    let t7880 = t2761 * t7875 * t332 * t7877 * t103;
    let t7920 = t2627 * t442;
    let t7921 = t919 * t7920;
    let t7927 = t442 * t818;
    let t7938 = t1087 * t2232;
    let t7939 = t1086 * t7938;
    (t7880, t7921, t7927, t7938, t7939)
}
