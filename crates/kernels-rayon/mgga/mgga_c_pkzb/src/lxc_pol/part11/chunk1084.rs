//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1084/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1084(t11817: f64, t204: f64, t334: f64, t1731: f64, t218: f64, t344: f64, t5555: f64, t847: f64, t16194: f64, t339: f64, t930: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18439 = t204 * t11817 * t334;
    let t18440 = 0.31310740740740740741e1_f64 * t18439;
    let t18442 = t218 * t1731 * t344;
    let t18443 = 0.13490888888888888889e1_f64 * t18442;
    let t18445 = t218 * t5555 * t847;
    let t18468 = 280.0_f64 / 81.0_f64 * t18439;
    let t18480 = 1.0_f64 / t339 / t16194 / t930 / 96.0_f64;
    let t18492 = f64::powf(t336, -0.25e1_f64);
    (t18439, t18440, t18442, t18443, t18445, t18468, t18480, t18492)
}
