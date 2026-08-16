//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1187/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1187(t10760: f64, t29951: f64, t6093: f64, t10872: f64, t12498: f64, t12492: f64, t19883: f64, t3179: f64, t3344: f64, t10879: f64, t12503: f64, t261: f64, t3304: f64, t9311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43547 = t6093 * t10760 * t29951;
    let t43549 = t10872 * t12498;
    let t43551 = t19883 * t12492;
    let t43553 = t3179 * t3344;
    let t43555 = t10879 * t12503;
    let t43559 = t3304 * t261 * t9311;
    (t43547, t43549, t43551, t43553, t43555, t43559)
}
