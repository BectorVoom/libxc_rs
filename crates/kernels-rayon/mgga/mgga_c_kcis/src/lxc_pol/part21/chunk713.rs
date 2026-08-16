//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 713/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk713(t7754: f64, t7755: f64, t2825: f64, t389: f64, t1096: f64, t1189: f64, t1021: f64, t1196: f64, t1200: f64, t7746: f64, t7750: f64, t7752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7756 = t7754 * t7755;
    let t7758 = t2825 * t389;
    let t7760 = t1096 * t1189;
    let t7762 = t1021 * t1196;
    let t7764 = t1021 * t1200;
    let t7766 = t7746 / 16.0_f64 - t7750 / 16.0_f64 - t7752 / 6.0_f64 + t7756 / 24.0_f64 - t7758 / 128.0_f64 + t7760 / 128.0_f64 + t7762 / 24.0_f64 - t7764 / 96.0_f64;
    (t7756, t7758, t7760, t7762, t7764, t7766)
}
