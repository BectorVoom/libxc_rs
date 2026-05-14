//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 923/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk923<F: Float>(t1083: F, t3524: F, t5804: F, t5802: F, t10833: F, t722: F, t10841: F, t703: F, t1979: F, t3525: F, t7483: F, t684: F, t1899: F, t1084: F, t9334: F, t2746: F, t3551: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10867 = t3524 * t1083;
    let t10868 = t10867 * t5804;
    let t10870 = 0.51726012919273400301e3 * t5802 * t10868;
    let t10873 = t10833 * t722;
    let t10878 = t10841 * t703;
    let t10887 = t10833 * t1979;
    let t10891 = 6.0 * t7483 * t3525;
    let t10892 = t10867 * t684;
    let t10894 = 6.0 * t1899 * t10892;
    let t10896 = 3.0 * t9334 * t1084;
    let t10898 = 3.0 * t2746 * t3551;
    (t10867, t10868, t10870, t10873, t10878, t10887, t10891, t10892, t10894, t10896, t10898)
}
