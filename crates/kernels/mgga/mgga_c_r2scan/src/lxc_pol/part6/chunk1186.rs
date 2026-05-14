//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1186/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1186<F: Float>(t1800: F, t5706: F, t650: F, t1647: F, t1882: F, t1891: F, t1893: F, t194: F, t1957: F, t1966: F, t1981: F, t21115: F, t21416: F, t21420: F, t21430: F, t21621: F, t21659: F, t21677: F, t21680: F, t21692: F, t21699: F, t21702: F, t224: F, t226: F, t5512: F, t5523: F, t5538: F, t5693: F, t5698: F, t5781: F, t5822: F, t63: F, t664: F, t687: F, t689: F, t690: F, t718: F, t76: F) -> (F, F, F) {
    let t21705 = 0.96491876992155210402e2 * t650 * t5706 * t1800;
    let t21709 = 0.3103560775156404018e4 * t1891 * t1882 * t1893 * t1647;
    let t21713 = -t21659 + 0.70178683471615754485e2 * t1981 * t226 * t21115 - 0.39929120607209281465e7 * t63 / t5693 / t194 * t5698 * t21416 - 0.57895126195293126243e3 * t1957 * t690 * t21420 + 0.12865583598954028054e3 * t687 * t5523 * t689 * t664 + 0.1078736821940706181e8 * t76 * t21677 * t224 * t21680 * t21115 + 24.0 * t5822 * t21621 * t664 + 0.10526802520742363173e2 * t718 * t226 * t21430 - 0.18216520838430511208e7 * t76 * t21692 * t5538 * t21115 + t21699 + t21702 - t21705 - t21709 - 144.0 * t5781 * t5512 * t1966;
    (t21705, t21709, t21713)
}
