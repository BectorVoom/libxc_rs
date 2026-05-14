//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 781/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk781<F: Float>(t5686: F, t650: F, t653: F, t685: F, t63: F, t688: F) -> (F, F, F, F, F, F) {
    let t5689 = 0.16081979498692535067e2 * t650 * t653 * t5686;
    let t5693 = t685 * t685;
    let t5694 = 1.0 / t5693;
    let t5695 = t63 * t5694;
    let t5696 = t688 * t688;
    let t5697 = 1.0 / t5696;
    (t5689, t5693, t5694, t5695, t5696, t5697)
}
