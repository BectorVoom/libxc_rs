//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 692/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk692(t28: f64, t265: f64, t504: f64, t7130: f64, t1081: f64, t1877: f64, t2057: f64, t2071: f64, t2522: f64, t52: f64, t607: f64, t6841: f64, t6848: f64, t7110: f64, t7114: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7150 = piecewise3(t505, 0.0_f64, t7130);
    let t7155 = piecewise3(t401, 3.0_f64 / 2.0_f64 * t2522 * t2057 * t6841 + t1877 * t7110 * t28 / 2.0_f64 - t1877 * t7114 * t6848 / 2.0_f64 + t1877 * t2057 * t1081 / 2.0_f64, -t2071 * t607 / 2.0_f64 + t7150 * t52 / 2.0_f64);
    (t7150, t7155)
}
