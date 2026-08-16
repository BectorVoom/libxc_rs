//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1396/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1396(t1340: f64, t40196: f64, t40192: f64, t40113: f64, t40169: f64, t40135: f64, t3869: f64, t39739: f64, t39430: f64, t39742: f64, t39440: f64, t39532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47088 = 0.35089341735807877242e1_f64 * t1340 * t40196;
    let t47092 = 0.14035736694323150897e2_f64 * t1340 * t40192;
    let t47096 = 0.51947577317044391277e2_f64 * t1340 * t40113;
    let t47098 = 0.91082604192152556044e5_f64 * t1340 * t40169;
    let t47109 = 0.6233709278045326953e3_f64 * t1340 * t40135;
    let t47116 = 0.86748650402413918736e-1_f64 * t3869 * t39739;
    let t47118 = 0.38527786510141256862e1_f64 * t3869 * t39430;
    let t47122 = 0.1301229756036208781e0_f64 * t3869 * t39742;
    let t47124 = 0.67471172535210825684e-1_f64 * t3869 * t39440;
    let t47131 = 0.21687162600603479684e-1_f64 * t3869 * t39532;
    (t47088, t47092, t47096, t47098, t47109, t47116, t47118, t47122, t47124, t47131)
}
