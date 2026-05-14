//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1424/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1424<F: Float>(t26881: F, t18869: F, t18872: F, t18875: F, t18878: F, t22592: F, t22596: F, t23696: F, t23697: F, t23698: F, t23699: F, t2049: F, t2055: F, t2820: F, t18888: F, t23700: F, t23701: F, t23702: F, t23705: F, t23706: F, t23707: F, t23708: F, t23710: F, t23973: F, t765: F) -> (F, F) {
    let t26882 = 0.1714584e0 * t26881;
    let t26883 = -0.2025780996e0 * t22592 - 0.4051561992e0 * t22596 + t23696 + t23697 + t23698 + t23699 + t18869 - t18872 - t18875 - t18878 - t26882;
    let t26886 = t2055 * t2820 * t2049;
    let t26890 = t23700 - t23701 - t23702 - t18888 - t23705 - t23706 - t23707 - 0.1714584e0 * t26886 + 0.675260332e-1 * t765 * t23973 - t23708 - t23710;
    (t26883, t26890)
}
