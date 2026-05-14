//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1414/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1414<F: Float>(t123: F, t2801: F, t35461: F, t2028: F, t33225: F, t33226: F, t7718: F, t1636: F, t24561: F, t117613: F, t117616: F, t117618: F, t121015: F, t121019: F, t23220: F, t34412: F, t34416: F, t34496: F, t34499: F, t34534: F, t34548: F, t34552: F, t9740: F, t9743: F) -> (F, F, F) {
    let t122418 = t2801 * t35461 * t123;
    let t122427 = t33225 * t33226 * t7718 * t2028;
    let t122435 = t33225 * t24561 * t1636;
    let t122446 = -0.17361111111111111111e-2 * t122418 * t9743 + 0.34722222222222222223e-2 * t34416 * t34548 - 0.92592592592592592593e-2 * t34412 * t34552 + 0.17361111111111111111e-2 * t9740 * t122427 + 0.34722222222222222222e-2 * t9740 * t33225 * t34499 * t23220 + 0.17361111111111111111e-2 * t9740 * t122435 + 0.34822083333333333332e-2 * t121015 - 0.11607361111111111111e-2 * t121019 - t117613 + t117616 - t117618 - 0.92592592592592592593e-2 * t34412 * t34534 - 0.92592592592592592593e-2 * t34412 * t34548 - 0.92592592592592592593e-2 * t34412 * t34496;
    (t122427, t122435, t122446)
}
