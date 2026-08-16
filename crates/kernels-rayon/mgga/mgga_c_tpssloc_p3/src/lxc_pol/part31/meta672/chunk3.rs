//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2016/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2016(t2085: f64, t6414: f64, t1352: f64, t19810: f64, t27078: f64, t5344: f64, t81047: f64, t84480: f64, t90889: f64, t90900: f64, t90903: f64, t93562: f64, t93572: f64, t96986: f64, t96989: f64, t96993: f64, t96997: f64, t97002: f64, t97007: f64, t97014: f64, t97017: f64) -> (f64, f64) {
    let t102587 = t2085 * t6414;
    let t102597 = -0.52089578783527170489e-1_f64 * t81047 - 0.16449340668482264365e-1_f64 * t96986 + 0.82246703342411321825e-2_f64 * t96989 + 0.19739208802178717238e0_f64 * t96993 + 0.3289868133696452873e-1_f64 * t96997 - t5344 * t102587 * t1352 - 2.0_f64 * t19810 * t27078 - 0.9869604401089358619e-1_f64 * t97002 - t90889 - 0.6579736267392905746e-1_f64 * t97007 - t93562 + 0.10417915756705434098e0_f64 * t90900 + t90903 - t93572 - 0.39478417604357434476e0_f64 * t97014 - 0.3289868133696452873e-1_f64 * t97017 - t84480;
    (t102587, t102597)
}
