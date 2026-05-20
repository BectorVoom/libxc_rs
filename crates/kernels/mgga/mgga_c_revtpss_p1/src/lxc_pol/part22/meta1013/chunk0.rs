//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3479/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3479<F: Float>(t19894: F, t3127: F, t3172: F, t15707: F, t15734: F, t19882: F, t3188: F, t16190: F, t4820: F, t1063: F, t19662: F, t19667: F) -> (F, F, F, F, F, F) {
    let t65444 = t3127 * t3172 * t19894;
    let t65446 = t15707 * t15734;
    let t65454 = t3188 * t19882;
    let t65456 = t16190 * t4820;
    let t65459 = t1063 * t3172 * t19662;
    let t65462 = t1063 * t3172 * t19667;
    (t65444, t65446, t65454, t65456, t65459, t65462)
}
