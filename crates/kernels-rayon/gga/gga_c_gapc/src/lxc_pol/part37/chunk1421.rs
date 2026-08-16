//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1421/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1421(t34873: f64, t34876: f64, t34884: f64, t34889: f64, t34894: f64, t34900: f64, t37111: f64, t37112: f64, t37114: f64, t37116: f64, t37118: f64, t34905: f64, t34907: f64, t34921: f64, t37124: f64, t37125: f64, t37126: f64, t37127: f64, t37129: f64, t37130: f64, t37131: f64, t37132: f64) -> (f64, f64) {
    let t38628 = -0.36231816839129402172e-6_f64 * t34873 - 0.44979384805509945073e-8_f64 * t34876 + t37111 + t37112 - 0.19666550313313802086e-7_f64 * t34884 + t37114 - 0.52389984474979915325e-8_f64 * t34889 - t37116 + 0.93149392396514289454e-9_f64 * t34894 + t37118 - 0.50595483470764842602e-7_f64 * t34900;
    let t38633 = -0.49166375783284505216e-8_f64 * t34905 + 0.65555167711046006954e-8_f64 * t34907 - t37124 + t37125 + t37126 + t37127 + 0.44935166661611007237e-6_f64 * t34921 - t37129 + t37130 - t37131 + t37132;
    (t38628, t38633)
}
