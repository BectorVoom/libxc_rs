//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 306/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk306(t322: f64, t1020: f64, t333: f64, t335: f64, t337: f64, t339: f64, t341: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t1022 = t333 * t1020;
    let t1024 = t335 * t1020;
    let t1026 = t337 * t1020;
    let t1028 = t339 * t1020;
    let t1030 = t341 * t1020;
    let t1035 = piecewise3(t332, t1012, 0.0_f64);
    (t1022, t1024, t1026, t1028, t1030, t1035)
}
