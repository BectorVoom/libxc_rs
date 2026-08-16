//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1041/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1041(t25: f64, t1409: f64, t1965: f64, t25398: f64, t25883: f64, t3966: f64, t40: f64, t607: f64, t6835: f64, t7643: f64, t28: f64, t870: f64, t4255: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t25890 = piecewise3(t115, t25398, t6835 * t1409 / 2.0_f64 + t1965 * t3966 / 2.0_f64 + t25883 * t40 / 2.0_f64 + t7643 * t607 / 2.0_f64);
    let t25891 = t870 * t28;
    let t25892 = t25891 * t4255;
    (t25890, t25892)
}
