//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1128/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1128(t10416: f64, t4278: f64, t3931: f64, t10412: f64, t3096: f64, t9199: f64, t11476: f64, t4231: f64, t9721: f64, t3053: f64, t9619: f64, t3055: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12482 = t4278 * t10416;
    let t12483 = t3931 * t12482;
    let t12486 = t4278 * t10412;
    let t12487 = t3931 * t12486;
    let t12490 = t3096 * t9199;
    let t12491 = t12490 * t11476;
    let t12492 = t3931 * t12491;
    let t12497 = t4231 * t9721;
    let t12498 = t3931 * t12497;
    let t12501 = t9619 * t3053;
    let t12502 = t4231 * t12501;
    let t12503 = t3931 * t12502;
    let t12506 = t4231 * t3055;
    (t12483, t12487, t12492, t12498, t12503, t12506)
}
