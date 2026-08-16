//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1101/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1101(t14668: f64, t8064: f64, t5036: f64, t8081: f64, t2189: f64, t6638: f64, t10498: f64, t1820: f64, t3330: f64, t6735: f64, t377: f64, t6681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29033 = 4.0_f64 * t14668 * t8064;
    let t29035 = 2.0_f64 * t5036 * t8081;
    let t29036 = t2189 * t6638;
    let t29038 = 6.0_f64 * t10498 * t29036;
    let t29039 = t8081 * t1820;
    let t29041 = 4.0_f64 * t3330 * t29039;
    let t29042 = t2189 * t6735;
    let t29044 = 2.0_f64 * t3330 * t29042;
    let t29045 = t6681 * t377;
    (t29033, t29035, t29036, t29038, t29039, t29041, t29042, t29044, t29045)
}
