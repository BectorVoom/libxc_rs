//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1266/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1266<F: Float>(t19398: F, t19400: F, t19341: F, t19388: F, t19394: F, t23783: F, t23785: F, t23788: F, t23794: F, t23796: F, t23798: F, t23800: F, t23801: F, t881: F, t2625: F, t2858: F, t6890: F) -> (F, F, F, F) {
    let t23802 = 0.73245789224026180215e-3 * t19398;
    let t23803 = 0.17090684152272775383e-2 * t19400;
    let t23804 = -0.7089e1 * t23783 - 0.2363e1 * t881 * t23785 - 0.7089e1 * t881 * t23788 + t19341 + t23794 + t23796 + t23798 + t23800 + t19388 + t19394 + t23801 - t23802 + t23803;
    let t23810 = 18.0 * t2858 * t6890 * t2625;
    (t23802, t23803, t23804, t23810)
}
