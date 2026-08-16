//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1280/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1280(t3330: f64, t5189: f64, t8081: f64, t26868: f64, t6735: f64, t28071: f64, t5036: f64, t14668: f64, t28002: f64, t29081: f64, t3325: f64, t1008: f64, t1704: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100945 = 4.0_f64 * t3330 * t8081 * t5189;
    let t100950 = t26868 * t6735;
    let t100952 = 2.0_f64 * t5036 * t28071;
    let t100954 = 4.0_f64 * t14668 * t28002;
    let t100957 = t3325 * t29081;
    let t100970 = t1704 * t1008;
    (t100945, t100950, t100952, t100954, t100957, t100970)
}
