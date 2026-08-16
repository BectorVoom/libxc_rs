//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 587/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk587<F: Float>(t5978: F, t827: F, t828: F, t124: F, t5962: F, t800: F, t5966: F, t2477: F, t190: F, t5825: F, t706: F, t5819: F) -> (F, F, F, F, F, F, F, F) {
    let t5980 = t827 * t828 * t5978;
    let t5984 = t124 * t5962;
    let t5985 = t800 * t5984;
    let t5988 = t124 * t5966;
    let t5989 = t800 * t5988;
    let t5993 = t2477 * t828 * t5966;
    let t5999 = t190 * t5825;
    let t6001 = F::cast_from(4.0_f64) * t706 * t5999;
    let t6002 = t190 * t5819;
    (t5980, t5984, t5985, t5989, t5993, t5999, t6001, t6002)
}
