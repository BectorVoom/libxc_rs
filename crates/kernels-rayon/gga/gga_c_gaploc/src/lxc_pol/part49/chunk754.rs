//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 754/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk754(t12837: f64, t2268: f64, t2765: f64, t3137: f64, t10283: f64, t921: f64, t3145: f64, t8045: f64, t2798: f64, t3207: f64, t1016: f64, t9243: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12838 = t2268 * t12837;
    let t12840 = t2765 * t3137;
    let t12842 = 0.85365019907028448797e-1_f64 * t2268 * t12840;
    let t12846 = t10283 * t921;
    let t12849 = 2.0_f64 * t8045 * t3145;
    let t12850 = t2798 * t3207;
    let t12851 = t9243 * t1016;
    (t12838, t12840, t12842, t12846, t12849, t12850, t12851)
}
