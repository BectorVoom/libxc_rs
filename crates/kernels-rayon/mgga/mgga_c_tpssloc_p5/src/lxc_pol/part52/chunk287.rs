//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 287/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk287(t25: f64, t265: f64, t394: f64, t1068: f64, t1070: f64, t193: f64, t336: f64, t873: f64, t890: f64, t916: f64, t956: f64, t958: f64, t963: f64, t396: f64, t40: f64, t606: f64, t607: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t1074 = piecewise3(t395, t1068 * t1070 * t193 * t336 - t890 + t916 + t956 + t958 - t963, t873);
    let t1079 = piecewise3(t115, t873 * t25 / 2.0_f64 + t265 * t606 / 2.0_f64, t1074 * t40 / 2.0_f64 + t396 * t607 / 2.0_f64);
    (t1074, t1079)
}
