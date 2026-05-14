//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1173/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1173<F: Float>(t1466: F, t7193: F, t1490: F, t303: F, t6928: F, t94453: F, t22219: F, t7931: F, t22224: F, t553: F, t1459: F, t7262: F, t7203: F, t29600: F, t7974: F, t28749: F, t28755: F, t95088: F, t98573: F, t99301: F) -> (F, F, F, F, F, F, F) {
    let t102462 = t7193 * t1466;
    let t102464 = t303 * t102462 * t1490;
    let t102467 = t303 * t94453 * t6928;
    let t102475 = t303 * t7931 * t22219;
    let t102478 = t303 * t553 * t22224;
    let t102481 = t303 * t1459 * t7262;
    let t102484 = t303 * t1459 * t7203;
    let t102486 = t29600 * t7974;
    let t102488 = t95088 - 0.17411041666666666666e-2 * t102464 + 0.34822083333333333332e-2 * t102467 + 0.61905925925925925925e-2 * t98573 + 0.23168402777777777778e-3 * t99301 * t28749 + 0.23168402777777777778e-3 * t99301 * t28755 - 0.17024129629629629629e-1 * t102475 + 0.11349419753086419753e-1 * t102478 - 0.61905925925925925925e-2 * t102481 + 0.11607361111111111111e-2 * t102484 - 0.11326774691358024691e-2 * t102486;
    (t102464, t102467, t102475, t102478, t102481, t102484, t102488)
}
