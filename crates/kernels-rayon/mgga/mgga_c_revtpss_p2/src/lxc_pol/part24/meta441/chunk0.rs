//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1397/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1397(t3869: f64, t39538: f64, t39427: f64, t39535: f64, t3853: f64, t3857: f64, t73: f64, t9940: f64, t820: f64, t843: f64, t9991: f64, t1386: f64, t2237: f64, t2482: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47138 = 0.43374325201206959368e-1_f64 * t3869 * t39538;
    let t47140 = 0.12842595503380418954e1_f64 * t3869 * t39427;
    let t47142 = 0.38025319932552508021e2_f64 * t3869 * t39535;
    let t47152 = 120.0_f64 * t3857 * t3853;
    let t47171 = t73 * t9940;
    let t47194 = t820 * t9991 * t843;
    let t47198 = t2482 * t1386 * t2237;
    (t47138, t47140, t47142, t47152, t47171, t47194, t47198)
}
