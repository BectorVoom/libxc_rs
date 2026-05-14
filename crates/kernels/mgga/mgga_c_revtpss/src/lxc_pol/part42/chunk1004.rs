//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1004/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1004<F: Float>(t1086: F, t3057: F, t3090: F, t11671: F, t3114: F, t11200: F, t225: F, t1053: F, t3204: F, t1021: F, t3201: F, t1054: F, t2434: F, t371: F, t373: F, t367: F) -> (F, F, F, F, F, F, F) {
    let t11926 = t3057 * t1086;
    let t11927 = t11926 * t3090;
    let t11933 = t3114 * t11671;
    let t11940 = t11200 * t225;
    let t11947 = t3204 * t1053;
    let t11956 = t1021 * t3201;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4 * t367 * t11970;
    (t11927, t11933, t11940, t11947, t11956, t11967, t11972)
}
