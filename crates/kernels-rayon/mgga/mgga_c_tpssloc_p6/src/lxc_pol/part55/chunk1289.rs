//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1289/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1289(t28: f64, t265: f64, t504: f64, t119677: f64, t125789: f64, t119784: f64, t1409: f64, t32566: f64, t34366: f64, t3966: f64, t52: f64, t607: f64, t8909: f64, t113: f64, t120002: f64, t120008: f64, t120019: f64, t123044: f64, t123119: f64, t123120: f64, t123122: f64, t123124: f64, t123126: f64, t123129: f64, t123138: f64, t123140: f64, t123142: f64, t1393: f64, t24932: f64, t27879: f64, t27888: f64, t34381: f64, t7266: f64, t7408: f64, t7983: f64, t7989: f64, t8329: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t125790 = piecewise3(t505, t125789, t119677);
    let t125797 = piecewise3(t401, t119784, t125790 * t52 / 2.0_f64 - t32566 * t1409 / 2.0_f64 - t34366 * t607 / 2.0_f64 - t8909 * t3966 / 2.0_f64);
    let t125802 = -2.0_f64 * t123119 + t34381 * t1393 + t120002 - t8329 - 4.0_f64 * t123120 - 4.0_f64 * t123122 - 4.0_f64 * t123124 - 4.0_f64 * t123126 - 4.0_f64 * t123129 - t120008 - 4.0_f64 * t123138 - 4.0_f64 * t123140 - 4.0_f64 * t123142 - t120019 - 4.0_f64 * t24932 * t7989 - 4.0_f64 * t27888 * t7989 - 4.0_f64 * t7266 * t27879 - t113 * (t123044 + t125797) - 2.0_f64 * t7983 * t7408;
    t125802
}
