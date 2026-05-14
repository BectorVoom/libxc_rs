//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 220/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk220<F: Float>(t964: F, t972: F, t973: F, t981: F, t902: F, t908: F, t341: F) -> (F, F, F, F, F) {
    let t983 = t964 * t972 * t973;
    let t985 = 0.5848223622634646207e0 * t981 * t983;
    let t986 = 0.83333333333333333333e-2 * t902;
    let t988 = -t986 - 0.83333333333333333333e-2 * t908;
    let t989 = t988 * t341;
    (t983, t985, t986, t988, t989)
}
