//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1253/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1253(t1408: f64, t2752: f64, t2: f64, t10143: f64, t606: f64, t1519: f64, t213: f64, t225: f64, t794: f64, t25051: f64, t254: f64, t853: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t86721 = t2752 * t1408;
    let t86730 = t2752 * t2;
    let t86770 = t10143 * t606;
    let t86873 = t213 * t1519 * t225;
    let t86893 = t794 * t1519;
    let t86988 = t25051 * t225;
    let t87013 = t853 * t254;
    (t86721, t86730, t86770, t86873, t86893, t86988, t87013)
}
