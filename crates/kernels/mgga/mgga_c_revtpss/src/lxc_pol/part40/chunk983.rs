//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 983/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk983<F: Float>(t207: F, t242: F, t240: F, t72: F, t136: F, t2476: F, t221: F, t2394: F, t2674: F, t231: F, t243: F, t2645: F, t2662: F, t2661: F, t2652: F, t2656: F) -> (F, F, F, F, F) {
    let t10696 = 1.0 / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    let t10703 = t2476 * t136;
    let t10705 = t10703 * t221 * t2394;
    let t10706 = t2674 * t10705;
    let t10709 = t243 * t2645 * t231;
    let t10710 = t2662 * t10709;
    let t10711 = t2661 * t10710;
    let t10713 = t2652 * t2656;
    (t10698, t10703, t10706, t10711, t10713)
}
