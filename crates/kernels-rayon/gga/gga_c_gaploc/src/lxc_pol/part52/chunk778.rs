//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 778/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk778(t10007: f64, t2530: f64, t825: f64, t9438: f64, t12705: f64, t7416: f64, t10012: f64, t2684: f64, t40820: f64, t900: f64, t22624: f64, t7427: f64) -> (f64, f64, f64, f64, f64) {
    let t41299 = t825 * t9438 * t10007 * t2530;
    let t41312 = t7416 * t12705;
    let t41316 = t2684 * t9438 * t10012 * t2530;
    let t41339 = t900 * t40820;
    let t41408 = t7427 * t9438 * t22624;
    (t41299, t41312, t41316, t41339, t41408)
}
