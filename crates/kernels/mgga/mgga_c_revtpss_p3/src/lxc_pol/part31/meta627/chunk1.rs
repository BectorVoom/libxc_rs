//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2081/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2081<F: Float>(t27543: F, t994: F, t1977: F, t3057: F, t1078: F, t11200: F, t7143: F, t15827: F, t27536: F, t15904: F, t25515: F, t12047: F) -> (F, F, F, F, F, F) {
    let t99947 = t994 * t27543;
    let t99953 = t3057 * t1977;
    let t99969 = t11200 * t7143 * t1078;
    let t99983 = F::cast_from(0.11433071498151929859e-2_f64) * t27536 * t15827;
    let t99984 = t25515 * t15904;
    let t99985 = t12047 * t99984;
    (t99947, t99953, t99969, t99983, t99984, t99985)
}
