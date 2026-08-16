//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1069/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1069(t26671: f64, t7719: f64, t1020: f64, t2630: f64, t4546: f64, t7718: f64, t1009: f64, t1071: f64, t4947: f64, t283: f64, t2836: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26672 = t26671 * t7719;
    let t26673 = t1020 * t26672;
    let t26675 = t4546 * t2630;
    let t26676 = t7718 * t26675;
    let t26677 = t1020 * t26676;
    let t26679 = t1009 * t1071;
    let t26680 = t26679 * t2630;
    let t26681 = t4947 * t26680;
    let t26684 = t2836 * t283;
    let t26685 = t26684 * t990;
    (t26672, t26673, t26675, t26676, t26677, t26679, t26680, t26681, t26684, t26685)
}
