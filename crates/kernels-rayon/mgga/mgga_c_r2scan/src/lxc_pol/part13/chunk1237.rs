//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1237/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1237(t11036: f64, t8370: f64, t8373: f64, t1070: f64, t23353: f64, t37041: f64, t11033: f64, t2391: f64, t37031: f64, t8367: f64, t3366: f64, t8355: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40833 = t11036 * t8370;
    let t40835 = t11036 * t8373;
    let t40837 = t23353 * t1070;
    let t40839 = 22.0_f64 / 9.0_f64 * t37041;
    let t40840 = t11033 * t2391;
    let t40841 = 2.0_f64 / 3.0_f64 * t40840;
    let t40842 = t37031 * t8367;
    let t40844 = t8355 * t3366;
    (t40833, t40835, t40837, t40839, t40841, t40842, t40844)
}
