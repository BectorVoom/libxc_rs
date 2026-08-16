//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 881/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk881(t5522: f64, t5812: f64, t7357: f64, t7516: f64, t9148: f64, t9163: f64, t3559: f64, t694: f64, t1096: f64, t1950: f64, t248: f64, t2796: f64, t2816: f64, t3565: f64, t3592: f64, t3605: f64, t5897: f64, t5903: f64, t704: f64, t7447: f64, t9345: f64, t9363: f64, t9365: f64, t9367: f64, t9392: f64, t9394: f64) -> (f64, f64, f64) {
    let t9515 = -t5812 + 0.22831111111111111111e-1_f64 * t5522 + 0.45662222222222222221e-1_f64 * t7357 - t7516 - 0.17123333333333333333e-1_f64 * t9148 + 0.5137e-1_f64 * t9163;
    let t9518 = t3559 * t694;
    let t9527 = -0.11696447245269292414e1_f64 * t5903 * t3592 + 0.5848223622634646207e0_f64 * t1950 * t3605 - 0.310907e-1_f64 * t9515 * t248 + t9345 - t9363 + t9365 - t9367 - t9392 - t9394 + 1.0_f64 * t9518 * t704 + 2.0_f64 * t7447 * t1096 + 2.0_f64 * t2796 * t2816 - 2.0_f64 * t5897 * t3565;
    (t9515, t9518, t9527)
}
