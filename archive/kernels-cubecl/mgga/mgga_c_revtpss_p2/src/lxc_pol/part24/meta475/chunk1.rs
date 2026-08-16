//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1459/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1459<F: Float>(t1063: F, t247: F, t42447: F, t6092: F, t3140: F, t6235: F, t3149: F, t11986: F, t6100: F, t11262: F, t3161: F, t6311: F) -> (F, F, F, F, F) {
    let t65292 = t1063 * t247 * t42447 * t6092;
    let t65338 = t6235 * t3140;
    let t65339 = t65338 * t3149;
    let t65357 = t1063 * t247 * t11986 * t6100;
    let t65581 = t3161 * t11262 * t6311;
    (t65292, t65338, t65339, t65357, t65581)
}
