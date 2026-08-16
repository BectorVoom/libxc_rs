//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 672/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk672(t1380: f64, t453: f64, t4811: f64, t234: f64, t1409: f64, t1497: f64, t454: f64, t452: f64, t4704: f64, t446: f64, t4854: f64, t4859: f64, t4862: f64) -> (f64, f64, f64, f64, f64) {
    let t4990 = t1380 * t4811 * t453;
    let t4991 = t234 * t4990;
    let t4992 = 0.35089341735807877242e1_f64 * t4991;
    let t4994 = t1497 * t1409 * t454;
    let t4995 = t234 * t4994;
    let t4996 = 0.35089341735807877242e1_f64 * t4995;
    let t4997 = t1380 * t452;
    let t4998 = t4997 * t4704;
    let t4999 = t234 * t4998;
    let t5000 = 0.51947577317044391277e2_f64 * t4999;
    let t5002 = t446 * t4854 * t453;
    let t5003 = t234 * t5002;
    let t5004 = 0.5848223622634646207e0_f64 * t5003;
    let t5006 = t4859 * t4811 * t4862;
    (t4992, t4996, t5000, t5004, t5006)
}
