//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1148/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1148(t1018: f64, t1079: f64, t11920: f64, t12622: f64, t12624: f64, t12627: f64, t12629: f64, t2405: f64, t2406: f64, t2951: f64, t2953: f64, t330: f64, t3381: f64, t3643: f64, t3645: f64, t42559: f64, t42592: f64, t837: f64, t838: f64, t9698: f64) -> f64 {
    let t42615 = (t42559 + t42592) * t330 + t12622 * t837 * t330 + 2.0_f64 * t11920 * t1018 * t330 + 2.0_f64 * t3643 * t2405 * t330 + 2.0_f64 * t12624 * t838 + t3381 * t2951 * t330 + t1079 * t9698 * t330 + t12627 * t838 + t3381 * t2953 * t330 + 2.0_f64 * t3645 * t2406 + t12629 * t838;
    t42615
}
