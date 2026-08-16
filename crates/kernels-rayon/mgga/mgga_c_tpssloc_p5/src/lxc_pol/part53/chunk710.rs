//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 710/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk710(t25: f64, t28: f64, t265: f64, t394: f64, t504: f64, t202: f64, t8743: f64, t8747: f64, t193: f64, t2752: f64, t870: f64, t1877: f64, t40: f64, t8744: f64, t8748: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t8753 = t202 * t8743;
    let t8756 = t202 * t8747;
    let t8759 = -t193 * t2752 * t8756 + t193 * t870 * t8753;
    let t8760 = piecewise3(t395, 0.0_f64, t8759);
    let t8763 = piecewise3(t115, t1877 * t8744 * t25 / 2.0_f64 - t1877 * t8748 * t25 / 2.0_f64, t8760 * t40 / 2.0_f64);
    let t8770 = piecewise3(t505, 0.0_f64, t8759);
    let t8773 = piecewise3(t401, t1877 * t8744 * t28 / 2.0_f64 - t1877 * t8748 * t28 / 2.0_f64, t8770 * t52 / 2.0_f64);
    (t8753, t8756, t8760, t8763, t8770, t8773)
}
