//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1166/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1166<F: Float>(t1830: F, t234: F, t5275: F, t1826: F, t5357: F, t584: F, t5861: F, t608: F, t18956: F, t171: F, t21074: F, t21077: F, t21080: F, t21083: F, t1416: F, t2036: F) -> (F, F, F, F, F, F) {
    let t22191 = 0.70178683471615754484e1 * t234 * t5275 * t1830;
    let t22194 = 0.46785788981077169656e1 * t234 * t1826 * t5357;
    let t22196 = t584 * t608 * t5861;
    let t22202 = 0.10849222222222222222e0 * t18956;
    let t22206 = 0.571528e-1 * t584 * t171 * (-0.4938888888888888889e-1 * t21074 + 0.11853333333333333334e0 * t21077 - 0.43901234567901234568e-1 * t21080 + 0.15365432098765432099e0 * t21083 + t22202);
    let t22210 = 120.0 * t1416 * t2036;
    (t22191, t22194, t22196, t22202, t22206, t22210)
}
