//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 888/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk888(t13300: f64, t3796: f64, t3482: f64, t1337: f64, t1404: f64, t1336: f64, t140: f64, t3800: f64, t3488: f64, t1299: f64, t3483: f64, t3487: f64) -> (f64, f64, f64, f64) {
    let t13301 = t3796 * t13300;
    let t13302 = t3482 * t13301;
    let t13304 = t1337 * t1404;
    let t13306 = t140 * t1336 * t13304;
    let t13307 = t13306 * t3800;
    let t13309 = t13306 * t3488;
    let t13311 = t3483 * t1299;
    let t13312 = t13311 * t3487;
    (t13302, t13307, t13309, t13312)
}
