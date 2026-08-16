//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1731/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1731(t28: f64, t265: f64, t504: f64, t26806: f64, t1409: f64, t2071: f64, t26861: f64, t3966: f64, t52: f64, t607: f64, t7150: f64, t7884: f64, t26814: f64, t19577: f64, t24432: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t26862 = piecewise3(t505, 0.0_f64, t26806);
    let t26869 = piecewise3(t401, t26861, -t7150 * t1409 / 2.0_f64 - t2071 * t3966 / 2.0_f64 + t26862 * t52 / 2.0_f64 - t7884 * t607 / 2.0_f64);
    let t26870 = t26814 + t26869;
    let t26872 = t24432 * t19577;
    (t26862, t26870, t26872)
}
