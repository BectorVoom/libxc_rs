//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1062/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1062(t21753: f64, t21808: f64, t1118: f64, t1099: f64, t11277: f64, t21723: f64, t11275: f64, t11136: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64) -> (f64, f64, f64) {
    let t21809 = t21753 + t21808;
    let t21810 = t21809 * t1118;
    let t21812 = 1.0_f64 * t1099 * t21810;
    let t21813 = t21723 * t11277;
    let t21815 = 0.51726012919273400301e3_f64 * t11275 * t21813;
    let t21826 = -t11136 + 0.12361111111111111111e-1_f64 * t14702 + 0.61805555555555555556e-2_f64 * t18203 - 0.18541666666666666667e-1_f64 * t18219 - 0.92708333333333333334e-2_f64 * t18229 + 0.10300925925925925926e-1_f64 * t21760 - 0.37083333333333333333e-1_f64 * t21764 - 0.18541666666666666666e-1_f64 * t21767 + 0.55625000000000000001e-1_f64 * t21771 + 0.55625000000000000001e-1_f64 * t21774 + 0.92708333333333333333e-2_f64 * t21778;
    (t21812, t21815, t21826)
}
