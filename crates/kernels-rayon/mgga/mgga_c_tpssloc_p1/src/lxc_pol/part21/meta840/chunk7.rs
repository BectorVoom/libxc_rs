//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3020/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3020(t25: f64, t265: f64, t394: f64, t59618: f64, t60840: f64, t60878: f64, t60904: f64, t60909: f64, t60924: f64, t60939: f64, t60962: f64, t63241: f64, t1074: f64, t12606: f64, t13493: f64, t1408: f64, t1409: f64, t14675: f64, t1642: f64, t16557: f64, t16558: f64, t17133: f64, t18176: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t396: f64, t3966: f64, t40: f64, t4705: f64, t47676: f64, t5397: f64, t5398: f64, t55677: f64, t5669: f64, t5955: f64, t59627: f64, t59629: f64, t59631: f64, t606: f64, t607: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t63245 = piecewise3(t395, t60840 + t60878 + t60904 + t60909 + t60924 + t60939 + t60962 + t63241, t59618);
    let t63261 = piecewise3(t115, t59618 * t25 / 2.0_f64 + t17133 * t606 + t5669 * t2249 / 2.0_f64 + t13493 * t1408 + t59627 + t59629 - t59631 + t2756 * t5397 / 2.0_f64 + t873 * t16557 + t47676, t63245 * t40 / 2.0_f64 + t18176 * t607 + t5955 * t2250 / 2.0_f64 + t14675 * t1409 + 2.0_f64 * t4705 * t3966 + t1642 * t12606 + t3220 * t5398 / 2.0_f64 + t1074 * t16558 + t396 * t55677 / 2.0_f64);
    t63261
}
