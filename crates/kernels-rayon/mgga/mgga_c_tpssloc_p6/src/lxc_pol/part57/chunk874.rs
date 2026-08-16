//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 874/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk874(t33334: f64, t533: f64, t1390: f64, t1983: f64, t7802: f64, t8526: f64, t2039: f64, t7670: f64, t2040: f64, t2096: f64, t24999: f64, t33133: f64, t33230: f64, t33233: f64, t33236: f64, t33238: f64, t33239: f64, t4028: f64, t6517: f64, t652: f64, t7458: f64, t7796: f64, t7806: f64, t8529: f64) -> (f64, f64, f64, f64) {
    let t33335 = t533 * t33334;
    let t33336 = t33335 * t1390;
    let t33337 = t1983 * t33336;
    let t33345 = 2.0_f64 * t8526 * t7802;
    let t33350 = t7670 * t2039;
    let t33354 = -2.0_f64 * t2040 * t24999 + t2096 * t33133 - 2.0_f64 * t33350 * t652 - 2.0_f64 * t4028 * t8529 - 2.0_f64 * t6517 * t7796 - 2.0_f64 * t6517 * t7806 - 2.0_f64 * t7458 * t8529 - t33230 - t33233 - t33236 - t33238 + t33239 + t33337 - t33345;
    (t33335, t33336, t33350, t33354)
}
