//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 976/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk976(t45: f64, t1163: f64, t3537: f64, t1338: f64, t3166: f64, t8006: f64, t8024: f64, t8035: f64, t1289: f64, t8050: f64, t2225: f64, t3431: f64, t10353: f64, t1985: f64, t1992: f64, t3575: f64, t581: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t10461 = t1163 * t3537;
    let t10464 = t3166 * t1338;
    let t10470 = 0.11696447245269292414e1_f64 * t8006;
    let t10471 = 2.0_f64 * t8024;
    let t10472 = 0.5848223622634646207e0_f64 * t8035;
    let t10473 = t8050 * t1289;
    let t10476 = t2225 * t3431;
    let t10484 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t10473 * t1985 + 8.0_f64 / 9.0_f64 * t10476 * t581 + 4.0_f64 / 9.0_f64 * t3575 * t1992 + 4.0_f64 / 3.0_f64 * t78 * t10353);
    (t10461, t10464, t10470, t10471, t10472, t10484)
}
