//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 986/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk986(t1103: f64, t2461: f64, t1053: f64, t1102: f64, t10653: f64, t10657: f64, t11478: f64, t11482: f64, t11485: f64, t11489: f64, t11492: f64, t11495: f64, t11500: f64, t11504: f64, t11508: f64, t11512: f64, t11566: f64, t11570: f64) -> (f64, f64) {
    let t11572 = t1103 * t2461;
    let t11574 = t1102 * t1053 * t11572;
    let t11577 = -0.15243824895787514157e-3_f64 * t11566 + 0.21684485328539747656e-4_f64 * t11570 + t11478 + t11482 - t11485 + t11489 - t11492 + 0.15243824895787514157e-3_f64 * t11574 + t11495 + t11500 - t11504 - t11508 - t11512 + 0.36021158228745895953e-3_f64 * t10653 - t10657;
    (t11572, t11577)
}
