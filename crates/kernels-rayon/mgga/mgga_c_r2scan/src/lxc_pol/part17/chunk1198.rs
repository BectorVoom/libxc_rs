//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1198/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1198(t42846: f64, t481: f64, t35373: f64, t795: f64, t792: f64, t9573: f64, t11550: f64, t983: f64, t10935: f64, t3162: f64, t3446: f64, t3453: f64, t9056: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43767 = t42846 * t481;
    let t43771 = t35373 * t795;
    let t43798 = t9573 * t792;
    let t43802 = t11550 * t983;
    let t43820 = t3446 * t10935 * t3162;
    let t43826 = t3446 * t3453 * t9056;
    (t43767, t43771, t43798, t43802, t43820, t43826)
}
