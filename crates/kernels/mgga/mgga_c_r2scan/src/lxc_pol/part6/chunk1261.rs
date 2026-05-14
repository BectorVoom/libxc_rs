//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1261/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1261<F: Float>(t18942: F, t1048: F, t2867: F, t6595: F, t18922: F, t18930: F, t18934: F, t18941: F, t18973: F, t18975: F, t18979: F, t19447: F, t19455: F, t19457: F, t23718: F, t2262: F, t7040: F) -> (F, F, F, F) {
    let t23719 = 96.0 * t18942;
    let t23724 = t1048 * t2867 * t6595;
    let t23725 = t18922 + t18930 + t23718 - t18934 + t18941 + t23719 - 0.2363e1 * t19447 - 0.7089e1 * t19455 - t18973 - 0.7089e1 * t19457 + t23724 - t18975 + t18979;
    let t23730 = 3.0 * t1048 * t7040 * t2262;
    (t23719, t23724, t23725, t23730)
}
