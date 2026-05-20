//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1461/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1461<F: Float>(t3201: F, t6318: F, t1011: F, t6292: F, t697: F, t19649: F, t372: F, t6284: F, t6288: F, t3091: F, t43240: F, t6267: F) -> (F, F, F, F, F, F) {
    let t66141 = t6318 * t3201;
    let t66218 = t1011 * t697 * t6292;
    let t66306 = t372 * t19649;
    let t66547 = t1011 * t697 * t6284;
    let t66721 = t1011 * t697 * t6288;
    let t66763 = t3091 * t43240 * t6267;
    (t66141, t66218, t66306, t66547, t66721, t66763)
}
