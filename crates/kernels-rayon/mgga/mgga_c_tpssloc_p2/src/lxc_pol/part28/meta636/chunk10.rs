//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2030/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2030(t91531: f64, t91548: f64, t12033: f64, t16022: f64, t26990: f64, t27115: f64, t3752: f64, t3882: f64, t568: f64, t7199: f64, t7214: f64, t7918: f64, t7937: f64, t81393: f64, t81395: f64, t84705: f64, t91505: f64) -> f64 {
    let t93899 = 0.52089578783527170489e-1_f64 * t91531;
    let t93906 = 0.3289868133696452873e-1_f64 * t91548;
    let t93914 = -t93899 - t12033 * t7937 - 2.0_f64 * t3882 * t27115 - 0.76763589786250567036e-1_f64 * t81393 + 4.0_f64 * t16022 * t7199 + t93906 + t3752 * t7918 * t568 - 2.0_f64 * t16022 * t7214 + 0.76763589786250567036e-1_f64 * t81395 - 12.0_f64 * t91505 * t26990 - t84705;
    t93914
}
