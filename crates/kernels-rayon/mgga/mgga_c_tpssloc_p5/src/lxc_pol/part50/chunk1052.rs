//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1052/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1052(t28: f64, t265: f64, t504: f64, t30952: f64, t30982: f64, t52: f64, t607: f64, t8435: f64, t30958: f64, t649: f64, t8319: f64, t510: f64, t1266: f64, t8320: f64, t8301: f64, t9231: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t30983 = piecewise3(t505, 0.0_f64, t30952);
    let t30988 = piecewise3(t401, t30982, t30983 * t52 / 2.0_f64 - t8435 * t607 / 2.0_f64);
    let t30989 = t30958 + t30988;
    let t30991 = t649 * t8319;
    let t30993 = 2.0_f64 * t30991 * t510;
    let t30995 = 2.0_f64 * t8320 * t1266;
    let t31000 = t9231 * t8301;
    (t30983, t30989, t30991, t30993, t30995, t31000)
}
