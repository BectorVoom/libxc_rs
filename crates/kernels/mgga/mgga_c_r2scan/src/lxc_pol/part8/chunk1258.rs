//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1258/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1258<F: Float>(t1814: F, t3142: F, t1810: F, t3033: F, t5249: F, t5252: F, t26671: F, t7693: F, t406: F, t8940: F, t41: F, t725: F, t8590: F, t1693: F, t3034: F, t1754: F, t8908: F) -> (F, F, F, F, F, F, F, F) {
    let t28778 = t3142 * t1814;
    let t28780 = t3142 * t1810;
    let t28783 = t5249 * t3033 * t5252;
    let t28785 = t7693 * t26671;
    let t28794 = t406 * t8940;
    let t28802 = t41 * t8590 * t725;
    let t28805 = t3034 * t1693;
    let t28808 = t8908 * t1754;
    (t28778, t28780, t28783, t28785, t28794, t28802, t28805, t28808)
}
