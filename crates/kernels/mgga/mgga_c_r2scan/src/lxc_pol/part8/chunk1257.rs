//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1257/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1257<F: Float>(t5: F, t736: F, t8590: F, t697: F, t1721: F, t3034: F, t1707: F, t1734: F, t8994: F, t8997: F, t1732: F, t745: F, t8967: F, t1831: F, t3142: F, t1838: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28665 = t8590 * t5 * t736;
    let t28728 = t8590 * t697;
    let t28730 = t3034 * t1721;
    let t28740 = t3034 * t1707;
    let t28744 = t8994 * t1734;
    let t28746 = t8997 * t1734;
    let t28748 = t8994 * t1732;
    let t28750 = t8967 * t745;
    let t28774 = t3142 * t1831;
    let t28776 = t3142 * t1838;
    (t28665, t28728, t28730, t28740, t28744, t28746, t28748, t28750, t28774, t28776)
}
