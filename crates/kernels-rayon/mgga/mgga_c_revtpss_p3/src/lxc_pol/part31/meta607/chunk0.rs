//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2045/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2045(t1889: f64, t94545: f64, t13846: f64, t13877: f64, t7021: f64, t5665: f64, t94497: f64, t14036: f64, t25997: f64, t13941: f64, t94423: f64, t14005: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98165 = t94545 * t1889;
    let t98168 = t7021 * t13846 * t13877;
    let t98169 = 7.0_f64 / 24.0_f64 * t98168;
    let t98174 = t94497 * t5665;
    let t98180 = t25997 * t14036;
    let t98181 = 0.50820002809285328226e-4_f64 * t98180;
    let t98185 = t94423 * t13941;
    let t98186 = 0.2032800112371413129e-3_f64 * t98185;
    let t98187 = t94423 * t14005;
    (t98165, t98169, t98174, t98181, t98186, t98187)
}
