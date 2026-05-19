//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 882/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk882<F: Float>(t1073: F, t2787: F, t470: F, t2790: F, t478: F, t7363: F, t475: F, t2791: F, t1080: F, t1090: F, t221: F, t222: F, t2771: F, t2774: F, t2789: F, t479: F, t492: F, t567: F, t7310: F, t7345: F, t7398: F, t7399: F, t7411: F, t7420: F, t7426: F, t7438: F, t7446: F, t7452: F, t7456: F, t7459: F, t7463: F, t7466: F) -> (F, F, F, F, F, F, F, F) {
    let t7468 = F::new(1.0) / t2787 / t1073;
    let t7469 = t470 * t7468;
    let t7471 = F::new(1.0) / t2790 / t478;
    let t7472 = t7363 * t7471;
    let t7476 = F::new(1.0) / t2787 / t475;
    let t7477 = t470 * t7476;
    let t7478 = t7363 * t2791;
    let t7481 = F::new(0.10274e0) * t222 * t567 * t2771 * t2774 - F::cast_from(0.10389515463408878255e3_f64) * t7398 * t7399 + F::cast_from(0.5848223622634646207e0_f64) * t1090 * t7411 + F::cast_from(0.16562821945185185185e-2_f64) * t221 * t7345 * t479 - t7310 + F::cast_from(0.56968947174242584612e-3_f64) * t221 * t7345 * t492 + F::cast_from(0.96491876992155210402e2_f64) * t2789 * t7420 * t1080 - t7426 - t7438 - t7446 + t7452 - t7456 + t7459 + t7463 + t7466 + F::cast_from(0.2069040516770936012e4_f64) * t7469 * t7472 - F::cast_from(0.19298375398431042081e3_f64) * t7477 * t7478;
    (t7468, t7469, t7471, t7472, t7476, t7477, t7478, t7481)
}
