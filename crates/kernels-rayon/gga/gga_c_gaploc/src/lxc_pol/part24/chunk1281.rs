//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1281/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1281(t326: f64, t32948: f64, t825: f64, t11109: f64, t5840: f64, t10856: f64, t2033: f64, t549: f64, t10811: f64, t7751: f64, t32893: f64, t10906: f64, t2013: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33067 = 0.18404604457881959845e2_f64 * t825 * t326 * t32948;
    let t33068 = t5840 * t11109;
    let t33069 = 0.51123901271894332902e0_f64 * t33068;
    let t33071 = t2033 * t549 * t10856;
    let t33072 = 0.59584149919750711116e-1_f64 * t33071;
    let t33074 = 0.42900587942220512003e1_f64 * t10811 * t7751;
    let t33077 = 0.92023022289409799224e1_f64 * t825 * t326 * t32893;
    let t33079 = 0.18404604457881959845e2_f64 * t2013 * t10906;
    (t33067, t33069, t33072, t33074, t33077, t33079)
}
