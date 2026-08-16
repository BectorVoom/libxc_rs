//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 373/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk373(t1629: f64, t187: f64, t2248: f64, t2254: f64, t2264: f64, t2268: f64, t633: f64, t119: f64, t32: f64, t5: f64, t645: f64, t88: f64) -> (f64, f64, f64) {
    let t2272 = t2248 - t2254 + t187 * (-t1629 * t2268 + t2264 * t633 - t2248 + t2254);
    let t2302 = 0.14764770444444444444e-2_f64 * t5 * t119 * t32;
    let t2303 = t88 * t645;
    (t2272, t2302, t2303)
}
