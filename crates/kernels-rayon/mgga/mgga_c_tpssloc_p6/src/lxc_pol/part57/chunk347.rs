//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 347/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk347(t25: f64, t28: f64, t265: f64, t394: f64, t504: f64, t2057: f64, t202: f64, t2056: f64, t193: f64, t870: f64, t1877: f64, t40: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t2058 = t2057 * t25;
    let t2061 = t202 * t2056;
    let t2063 = t193 * t2061 * t870;
    let t2064 = piecewise3(t395, 0.0_f64, t2063);
    let t2067 = piecewise3(t115, t1877 * t2058 / 2.0_f64, t2064 * t40 / 2.0_f64);
    let t2068 = t2057 * t28;
    let t2071 = piecewise3(t505, 0.0_f64, t2063);
    let t2074 = piecewise3(t401, t1877 * t2068 / 2.0_f64, t2071 * t52 / 2.0_f64);
    let t2075 = t2067 + t2074;
    (t2061, t2064, t2071, t2075)
}
