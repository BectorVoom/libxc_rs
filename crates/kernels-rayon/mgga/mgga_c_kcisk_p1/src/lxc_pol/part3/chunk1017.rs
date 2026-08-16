//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1017/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1017(t529: f64, t1287: f64, t13778: f64, t13785: f64, t15016: f64, t15032: f64, t1558: f64, t382: f64, t4144: f64, t4148: f64, t4354: f64, t525: f64, t526: f64, t6442: f64) -> f64 {
    let t530 = t529 < -0.66725e-1_f64;
    let t15039 = piecewise3(t530, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t15016 * t382 - 10.0_f64 / 9.0_f64 * t525 * t4354 * t1287 + 40.0_f64 / 27.0_f64 * t525 * t1558 * t4144 - 10.0_f64 / 9.0_f64 * t525 * t1558 * t4148 - 280.0_f64 / 243.0_f64 * t525 * t526 * t13778 + 40.0_f64 / 27.0_f64 * t6442 * t15032 - 10.0_f64 / 27.0_f64 * t525 * t526 * t13785);
    t15039
}
