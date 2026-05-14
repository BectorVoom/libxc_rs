//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 844/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk844<F: Float>(t102: F, t1504: F, t978: F, t5825: F, t967: F, t120: F, t8102: F, t506: F, t1243: F, t2863: F, t542: F, t974: F, t496: F, t127: F, t5810: F, t5819: F, t5836: F, t8181: F, t8182: F) -> (F, F, F, F, F) {
    let t8186 = 0.1753815e2 * t102 * t978 * t1504;
    let t8187 = t5825 * t967;
    let t8191 = t120 * t8102;
    let t8193 = 0.2923025e1 * t102 * t8191;
    let t8194 = t506 * t8102;
    let t8197 = t2863 * t1243;
    let t8198 = 0.64956111111111111111e0 * t8197;
    let t8199 = t542 * t974;
    let t8200 = t496 * t8199;
    let t8202 = -0.195872e1 * t5810 - t8181 - t8182 - t5819 / 2.0 - t8186 - 0.293808e2 * t127 * t8187 * t1504 - t8193 - 0.146904e1 * t127 * t8194 - t8198 - 2.0 / 9.0 * t8200 + t5836;
    (t8186, t8193, t8198, t8199, t8202)
}
