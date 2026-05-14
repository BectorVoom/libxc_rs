//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1379/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1379<F: Float>(t22350: F, t22352: F, t22354: F, t26690: F, t26693: F, t26695: F, t26700: F, t28728: F, t28730: F, t28740: F, t28744: F, t28746: F, t28748: F, t22365: F, t26704: F, t26706: F, t26708: F, t26710: F, t26712: F, t26721: F, t26725: F, t26729: F, t26730: F, t26732: F, t26737: F, t26739: F, t28750: F) -> (F, F) {
    let t33620 = -0.48024514811839999998e-1 * t26690 - 0.10805515832664e0 * t26693 + 0.19518446340543131715e0 * t28728 + 0.19518446340543131715e0 * t28730 + 0.11711067804325879029e1 * t26695 - t22350 + t22352 - t22354 + 0.57791679765211885291e1 * t28740 + t26700 + 0.16008171603946666667e-1 * t28744 + 0.80040858019733333332e-2 * t28746 - 0.600306435148e-2 * t28748;
    let t33630 = 0.17544670867903938621e1 * t28750 + 0.86687519647817827941e1 * t26704 - 0.29277669510814697574e0 * t26706 + 0.80040858019733333331e-2 * t26708 + 0.80040858019733333331e-2 * t26710 + 0.1200612870296e-1 * t26712 - t26721 + 0.5143752e0 * t22365 - t26725 + t26729 + 0.92286169723947659916e4 * t26730 + 0.27324781257645766812e6 * t26732 - t26737 - t26739;
    (t33620, t33630)
}
