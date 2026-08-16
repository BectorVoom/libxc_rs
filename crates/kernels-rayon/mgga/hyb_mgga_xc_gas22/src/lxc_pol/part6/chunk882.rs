//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 882/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk882(t1073: f64, t2787: f64, t470: f64, t2790: f64, t478: f64, t7363: f64, t475: f64, t2791: f64, t1080: f64, t1090: f64, t221: f64, t222: f64, t2771: f64, t2774: f64, t2789: f64, t479: f64, t492: f64, t567: f64, t7310: f64, t7345: f64, t7398: f64, t7399: f64, t7411: f64, t7420: f64, t7426: f64, t7438: f64, t7446: f64, t7452: f64, t7456: f64, t7459: f64, t7463: f64, t7466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7468 = 1.0_f64 / t2787 / t1073;
    let t7469 = t470 * t7468;
    let t7471 = 1.0_f64 / t2790 / t478;
    let t7472 = t7363 * t7471;
    let t7476 = 1.0_f64 / t2787 / t475;
    let t7477 = t470 * t7476;
    let t7478 = t7363 * t2791;
    let t7481 = 0.10274e0_f64 * t222 * t567 * t2771 * t2774 - 0.10389515463408878255e3_f64 * t7398 * t7399 + 0.5848223622634646207e0_f64 * t1090 * t7411 + 0.16562821945185185185e-2_f64 * t221 * t7345 * t479 - t7310 + 0.56968947174242584612e-3_f64 * t221 * t7345 * t492 + 0.96491876992155210402e2_f64 * t2789 * t7420 * t1080 - t7426 - t7438 - t7446 + t7452 - t7456 + t7459 + t7463 + t7466 + 0.2069040516770936012e4_f64 * t7469 * t7472 - 0.19298375398431042081e3_f64 * t7477 * t7478;
    (t7468, t7469, t7471, t7472, t7476, t7477, t7478, t7481)
}
