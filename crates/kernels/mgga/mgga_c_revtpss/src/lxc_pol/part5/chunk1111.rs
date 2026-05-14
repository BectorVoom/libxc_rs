//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1111/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1111<F: Float>(t15123: F, t15125: F, t15301: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t19202: F, t19224: F) -> (F,) {
    let t19226 = -0.57386111111111111112e0 * t18906 + 0.20659e1 * t18911 - 0.68863333333333333334e0 * t18915 + 0.6311625e0 * t18951 - 0.23154444444444444445e0 * t15123 - 0.68863333333333333332e0 * t15125 + t15301 - 0.309885e1 * t18928 + 0.20659e1 * t18932 - 0.34431666666666666667e0 * t18939 + t19202 + 0.264729375e1 * t18980 - 0.3529725e1 * t18982 - 0.17648625e1 * t18985 - 0.157790625e0 * t18988 + 0.6311625e0 * t18990 + 0.31558125e0 * t18993 + 0.3529725e1 * t18995 + 0.11477222222222222222e0 * t18919 - 0.34431666666666666667e0 * t18924 + 0.17215833333333333333e0 * t18934 + t19224;
    (t19226,)
}
