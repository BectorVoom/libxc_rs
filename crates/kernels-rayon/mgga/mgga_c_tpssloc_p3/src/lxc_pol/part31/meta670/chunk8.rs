//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1997/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1997(t28: f64, t265: f64, t504: f64, t101938: f64, t101981: f64, t102012: f64, t102048: f64, t102087: f64, t1409: f64, t16558: f64, t2071: f64, t26862: f64, t29189: f64, t3966: f64, t52: f64, t5398: f64, t607: f64, t7150: f64, t7884: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t102090 = piecewise3(t505, 0.0_f64, t101938);
    let t102102 = piecewise3(t401, t101981 + t102012 + t102048 + t102087, t102090 * t52 / 2.0_f64 - t29189 * t607 / 2.0_f64 - t26862 * t1409 - t7884 * t3966 - t7150 * t5398 / 2.0_f64 - t2071 * t16558 / 2.0_f64);
    t102102
}
