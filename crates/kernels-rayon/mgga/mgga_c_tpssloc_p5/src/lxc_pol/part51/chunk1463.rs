//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1463/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1463(t27188: f64, t6534: f64, t121004: f64, t1873: f64, t121007: f64, t33234: f64, t23938: f64, t7467: f64, t26977: f64, t26135: f64, t7042: f64, t120121: f64, t120123: f64, t120125: f64, t120131: f64, t120145: f64, t120148: f64, t121129: f64, t2039: f64, t22461: f64, t31237: f64, t31239: f64, t33085: f64, t7056: f64, t7801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t122734 = t27188 * t6534;
    let t122735 = t121004 * t1873;
    let t122736 = t121007 * t1873;
    let t122737 = t33234 * t6534;
    let t122738 = t23938 * t7467;
    let t122739 = t26977 * t7467;
    let t122740 = t7042 * t26135;
    let t122754 = 2.0_f64 * t120145 * t2039 + 2.0_f64 * t120148 * t2039 + 2.0_f64 * t22461 * t7801 + 2.0_f64 * t33085 * t7056 + t120121 + t120123 + t120125 + t120131 + t121129 + t31237 + t31239;
    (t122734, t122735, t122736, t122737, t122738, t122739, t122740, t122754)
}
