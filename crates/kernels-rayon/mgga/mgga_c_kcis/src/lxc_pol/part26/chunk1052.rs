//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1052/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1052(t2490: f64, t26544: f64, t7627: f64, t774: f64, t755: f64, t2150: f64, t2526: f64, t2622: f64, t62: f64, t157: f64, t26521: f64, t26523: f64, t26525: f64, t26528: f64, t26531: f64, t26534: f64, t26536: f64, t26538: f64, t26540: f64, t26542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26545 = t2490 * t26544;
    let t26547 = t7627 * t774;
    let t26548 = t755 * t26547;
    let t26550 = t2150 * t2526;
    let t26551 = t755 * t26550;
    let t26553 = t62 * t2622;
    let t26554 = t157 * t26553;
    let t26556 = 0.1875e0_f64 * t26521 - 0.375e0_f64 * t26523 - 0.75e0_f64 * t26525 + 0.375e0_f64 * t26528 + 0.75e0_f64 * t26531 - 0.1875e0_f64 * t26534 + 0.1125e1_f64 * t26536 - 0.809375e-1_f64 * t26538 + 0.161875e0_f64 * t26540 + 0.6475e0_f64 * t26542 - 0.161875e0_f64 * t26545 - 0.6475e0_f64 * t26548 + 0.809375e-1_f64 * t26551 - 0.161875e1_f64 * t26554;
    (t26545, t26547, t26548, t26550, t26551, t26553, t26554, t26556)
}
