//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 698/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk698(t5: f64, t6889: f64, t8621: f64, t1985: f64, t1998: f64, t2085: f64, t214: f64, t590: f64, t60: f64, t131: f64, t8308: f64, t8302: f64, t112: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t8622 = t6889 * t8621;
    let t8623 = t1985 * t8622;
    let t8630 = t1998 * t2085;
    let t8631 = t214 * t8630;
    let t8632 = t1985 * t8631;
    let t8705 = 1.0_f64 / t60 / t590;
    let t8706 = t8705 * t131;
    let t8707 = t8706 * t8308;
    let t8710 = piecewise3(t8, 0.0_f64, 5.0_f64 / 36.0_f64 * t8302 * t8707);
    let t8711 = t8710 * t112;
    (t8622, t8623, t8630, t8631, t8632, t8705, t8706, t8707, t8710, t8711)
}
