//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 848/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk848(t28: f64, t1409: f64, t2161: f64, t28802: f64, t29840: f64, t52: f64, t5398: f64, t8097: f64, t29514: f64, t2165: f64, t5493: f64, t113: f64, t1442: f64, t1774: f64, t28815: f64, t28819: f64, t28822: f64, t28825: f64, t28829: f64, t28833: f64, t28837: f64, t28841: f64, t28843: f64, t28861: f64, t28863: f64, t28866: f64, t29493: f64, t4028: f64, t510: f64, t5450: f64, t5457: f64, t652: f64, t7983: f64, t7989: f64, t8103: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t29847 = piecewise3(t401, t28802, t29840 * t52 / 2.0_f64 - t8097 * t1409 - t2161 * t5398 / 2.0_f64);
    let t29848 = t29514 + t29847;
    let t29855 = t2165 * t5493;
    let t29864 = -t113 * t29848 - 2.0_f64 * t1442 * t8103 - 2.0_f64 * t1774 * t7983 - t2165 * t5450 - 2.0_f64 * t2165 * t5457 - 2.0_f64 * t29493 * t510 - 2.0_f64 * t29855 * t652 - 4.0_f64 * t4028 * t7989 - t28815 + t28819 + t28822 + t28825 + t28829 - t28833 + t28837 + t28841 + t28843 - t28861 - t28863 - t28866;
    (t29848, t29855, t29864)
}
