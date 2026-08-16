//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2353/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2353(t1395: f64, t8119: f64, t1858: f64, t7415: f64, t27930: f64, t576: f64, t112: f64, t27907: f64, t2169: f64, t2319: f64, t1458: f64, t16538: f64, t2363: f64, t24969: f64, t24972: f64, t27921: f64, t4072: f64, t577: f64, t671: f64, t85423: f64, t86582: f64, t86606: f64, t86610: f64, t86612: f64, t86614: f64, t86616: f64, t86619: f64, t86622: f64, t86625: f64, t86629: f64, t96277: f64) -> (f64, f64, f64, f64) {
    let t96300 = 2.0_f64 * t1395 * t8119;
    let t96303 = 2.0_f64 * t7415 * t1858;
    let t96308 = 2.0_f64 * t576 * t27930;
    let t96311 = t27907 * t112;
    let t96316 = t2169 * t2319;
    let t96327 = 27.0_f64 * t96311 * t671 + 0.135e2_f64 * t27921 * t2363 + 27.0_f64 * t96316 * t1458 + 0.45e1_f64 * t96277 * t577 + t86582 + t86606 + 0.135e2_f64 * t85423 * t1458 + 27.0_f64 * t24969 * t4072 + t86610 + t86612 + t86614 + t86616 + t86619 + t86622 + t86625 + t86629 + 54.0_f64 * t24972 * t16538;
    (t96300, t96303, t96308, t96327)
}
