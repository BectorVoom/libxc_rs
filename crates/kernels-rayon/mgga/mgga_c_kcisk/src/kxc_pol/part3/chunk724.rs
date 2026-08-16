//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 724/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk724(t1862: f64, t5060: f64, t5064: f64, t1869: f64, t4811: f64, t5065: f64, t1689: f64, t4822: f64, t139: f64, t5911: f64, t710: f64, t3521: f64, t4606: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11236 = t1862 * t5060;
    let t11237 = t11236 * sigma2;
    let t11238 = t11237 * t5064;
    let t11239 = t1869 * t11238;
    let t11241 = t4811 * t5065;
    let t11245 = t1689 * t4822;
    let t11250 = t139 * t5911;
    let t11252 = 0.29201909629629629629e-3_f64 * t11250 * t710;
    let t11255 = t3521 * t4606;
    (t11236, t11239, t11241, t11245, t11250, t11252, t11255)
}
