//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1042/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1042<F: Float>(t2476: F, t5966: F, t236: F, t807: F, t5819: F, t633: F, t637: F, t221: F, t2675: F, t5962: F, t2674: F, t243: F, t6016: F) -> (F, F, F, F, F, F, F, F) {
    let t18352 = t2476 * t5966;
    let t18353 = t236 * t18352;
    let t18354 = t807 * t18353;
    let t18367 = t633 * t5819;
    let t18379 = t637 * t5819;
    let t18402 = t2675 * t221 * t5962;
    let t18403 = t2674 * t18402;
    let t18408 = t243 * t6016;
    (t18352, t18353, t18354, t18367, t18379, t18402, t18403, t18408)
}
