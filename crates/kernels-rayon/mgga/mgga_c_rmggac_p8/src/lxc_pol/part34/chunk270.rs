//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 270/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk270(t2123: f64, t338: f64, t118: f64, t2055: f64, t2058: f64, t2062: f64, t2066: f64, t2071: f64, t2076: f64, t2082: f64, t2087: f64, t2088: f64, t2090: f64, t2092: f64) -> (f64, f64, f64) {
    let t2124 = t338 * t2123;
    let t2125 = t118 * t2124;
    let t2127 = 0.2993560425465952141e-1_f64 * t2055 - 0.44903406381989282115e-1_f64 * t2058 - 0.14967802127329760705e-1_f64 * t2062 - t2066 - 0.10227998120342003148e-1_f64 * t2071 + 0.13637330827122670864e-1_f64 * t2076 + 0.34093327067806677161e-2_f64 * t2082 + t2087 + 0.59871208509319042821e-1_f64 * t2088 - 0.59871208509319042821e-1_f64 * t2090 - 0.39914139006212695214e-1_f64 * t2092 + 0.19957069503106347607e-1_f64 * t2125;
    (t2124, t2125, t2127)
}
