//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1354/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1354(t10496: f64, t540: f64, t10122: f64, t1328: f64, t1445: f64, t1584: f64, t31715: f64, t31870: f64, t34119: f64, t34121: f64, t34123: f64, t34125: f64, t34128: f64, t34131: f64, t34143: f64, t34145: f64, t34148: f64, t34151: f64, t34153: f64, t34156: f64, t536: f64, t574: f64, t597: f64) -> f64 {
    let t34157 = t10496 * t540;
    let t34160 = -0.46011511144704899612e1_f64 * t1584 * t1445 * t31870 + t34119 + t34121 + t34123 + t34125 + t34128 + 0.71500979903700853338e0_f64 * t536 * t34131 - 0.92023022289409799224e1_f64 * t574 * t1445 * t10122 * t1328 + 0.43710935587469654631e2_f64 * t597 * t1445 * t31715 - t34143 - t34145 - t34148 - t34151 - t34153 - t34156 + 0.47667319935800568892e0_f64 * t536 * t34157;
    t34160
}
