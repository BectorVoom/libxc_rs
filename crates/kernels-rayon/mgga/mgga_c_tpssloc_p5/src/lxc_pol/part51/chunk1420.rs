//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1420/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1420(t113: f64, t121958: f64, t122082: f64, t121231: f64, t121233: f64, t121234: f64, t121237: f64, t121240: f64, t121253: f64, t121254: f64, t2039: f64, t2075: f64, t2314: f64, t24983: f64, t25958: f64, t26098: f64, t31734: f64, t33350: f64, t4034: f64, t652: f64, t7042: f64, t7458: f64) -> f64 {
    let t122084 = t113 * (t121958 + t122082);
    let t122085 = -2.0_f64 * t2039 * t25958 * t652 - t2075 * t26098 - 2.0_f64 * t2314 * t33350 - 2.0_f64 * t24983 * t7042 - 2.0_f64 * t31734 * t7458 - 2.0_f64 * t33350 * t4034 - t121231 - t121233 - t121234 - t121237 - t121240 - t121253 - t121254 - t122084;
    t122085
}
