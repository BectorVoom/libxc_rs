//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1224/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1224(t6817: f64, t969: f64, t3034: f64, t6423: f64, t1219: f64, t6789: f64, t1831: f64, t5233: f64, t6808: f64, t6805: f64, t4758: f64, t5253: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20475 = t6817 * t969;
    let t20478 = t6423 * t3034;
    let t20479 = t20478 * t969;
    let t20486 = t6789 * t1219;
    let t20489 = t1831 * t5233;
    let t20492 = t6808 * t1219;
    let t20495 = t6805 * t1219;
    let t20498 = t5253 * t4758;
    (t20475, t20479, t20486, t20489, t20492, t20495, t20498)
}
