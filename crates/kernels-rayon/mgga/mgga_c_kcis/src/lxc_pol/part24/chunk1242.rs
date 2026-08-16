//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1242/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1242(t1020: f64, t8047: f64, t96476: f64, t1250: f64, t20684: f64, t251: f64, t15573: f64, t29126: f64, t7788: f64, t15171: f64, t1662: f64, t5310: f64) -> (f64, f64, f64, f64, f64) {
    let t100264 = t1020 * t96476 * t8047;
    let t100268 = t20684 * t251 * t1250;
    let t100275 = t15573 * t29126;
    let t100276 = t7788 * t100275;
    let t100280 = t5310 * t15171 * t1662;
    (t100264, t100268, t100275, t100276, t100280)
}
