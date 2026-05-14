//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 782/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk782<F: Float>(t11921: F, t828: F, t1086: F, t3057: F, t3090: F, t11200: F, t225: F, t366: F, t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F, t1035: F, t11239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11922 = t828 * t11921;
    let t11926 = t3057 * t1086;
    let t11927 = t11926 * t3090;
    let t11940 = t11200 * t225;
    let t11941 = t11940 * t366;
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4 * t367 * t11970;
    let t11986 = t675 * t1065;
    let t12046 = t11239 * t1035;
    (t11922, t11926, t11927, t11940, t11941, t11970, t11972, t11986, t12046)
}
