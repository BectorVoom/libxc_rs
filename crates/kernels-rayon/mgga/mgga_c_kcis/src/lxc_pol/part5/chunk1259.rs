//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1259/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1259(t1396: f64, t21020: f64, t1395: f64, t1394: f64, t5477: f64, t5632: f64, t5780: f64, t3738: f64, t6913: f64, t5628: f64, t5748: f64, t1464: f64) -> (f64, f64, f64, f64) {
    let t21021 = t1396 * t21020;
    let t21022 = t1395 * t21021;
    let t21023 = t1394 * t21022;
    let t21025 = t5632 * t5477;
    let t21026 = t1395 * t21025;
    let t21027 = t5780 * t21026;
    let t21029 = t3738 * t6913;
    let t21030 = t1394 * t21029;
    let t21032 = t5748 * t5628;
    let t21033 = t1464 * t21032;
    (t21023, t21027, t21030, t21033)
}
