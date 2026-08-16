//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 992/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk992(t1083: f64, t3524: f64, t5804: f64, t5802: f64, t10833: f64, t722: f64, t10841: f64, t703: f64, t1979: f64, t3525: f64, t7483: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10867 = t3524 * t1083;
    let t10868 = t10867 * t5804;
    let t10870 = 0.51726012919273400301e3_f64 * t5802 * t10868;
    let t10873 = t10833 * t722;
    let t10878 = t10841 * t703;
    let t10887 = t10833 * t1979;
    let t10891 = 6.0_f64 * t7483 * t3525;
    let t10892 = t10867 * t684;
    (t10867, t10868, t10870, t10873, t10878, t10887, t10891, t10892)
}
