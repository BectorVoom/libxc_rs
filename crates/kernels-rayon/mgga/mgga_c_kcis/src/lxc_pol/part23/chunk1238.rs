//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1238/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1238(t1386: f64, t5732: f64, t3801: f64, t5709: f64, t5885: f64, t28510: f64, t4142: f64, t1889: f64, t94228: f64, t94229: f64, t3717: f64, t52460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98205 = t1386 * t5732;
    let t98220 = t5709 * t5885 * t3801;
    let t98225 = t4142 * t28510;
    let t98226 = 0.14739506172839506172e-2_f64 * t98225;
    let t98230 = t94228 * t1889 * t94229;
    let t98233 = t52460 * t3717;
    (t98205, t98220, t98225, t98226, t98230, t98233)
}
