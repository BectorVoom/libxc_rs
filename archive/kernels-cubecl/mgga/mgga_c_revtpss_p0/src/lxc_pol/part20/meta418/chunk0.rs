//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1560/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1560<F: Float>(t2439: F, t3421: F, t12278: F, t698: F, t12274: F, t12256: F, t39443: F, t141: F, t3417: F, t12268: F, t1145: F, t1121: F, t39457: F) -> (F, F, F, F, F, F, F, F) {
    let t43783 = t2439 * t3421;
    let t43785 = t698 * t12278;
    let t43787 = t698 * t12274;
    let t43789 = t12256 * t39443;
    let t43791 = t141 * t3417 * t43789;
    let t43793 = t12268 * t39443;
    let t43795 = t141 * t1145 * t43793;
    let t43797 = t1121 * t39457;
    (t43783, t43785, t43787, t43789, t43791, t43793, t43795, t43797)
}
