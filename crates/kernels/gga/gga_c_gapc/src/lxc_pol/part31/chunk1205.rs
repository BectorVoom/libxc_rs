//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1205/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1205<F: Float>(t34873: F, t34876: F, t34884: F, t34889: F, t34894: F, t34900: F, t37111: F, t37112: F, t37114: F, t37116: F, t37118: F, t34905: F, t34907: F, t34921: F, t37124: F, t37125: F, t37126: F, t37127: F, t37129: F, t37130: F, t37131: F, t37132: F) -> (F, F) {
    let t38628 = -0.36231816839129402172e-6 * t34873 - 0.44979384805509945073e-8 * t34876 + t37111 + t37112 - 0.19666550313313802086e-7 * t34884 + t37114 - 0.52389984474979915325e-8 * t34889 - t37116 + 0.93149392396514289454e-9 * t34894 + t37118 - 0.50595483470764842602e-7 * t34900;
    let t38633 = -0.49166375783284505216e-8 * t34905 + 0.65555167711046006954e-8 * t34907 - t37124 + t37125 + t37126 + t37127 + 0.44935166661611007237e-6 * t34921 - t37129 + t37130 - t37131 + t37132;
    (t38628, t38633)
}
