//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 967/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk967(t13315: f64, t13364: f64, t13411: f64, t13461: f64, t14159: f64, t14197: f64, t14239: f64, t14279: f64, t504: f64, t1458: f64, t4163: f64, t1520: f64) -> (f64, f64) {
    let t14282 = t13315 + t13364 + t13411 + t13461 + t14159 + t14197 + t14239 + t14279;
    let t14283 = t14282 * t504;
    let t14284 = t4163 * t1458;
    let t14286 = 3.0_f64 * t14284 * t1520;
    (t14283, t14286)
}
