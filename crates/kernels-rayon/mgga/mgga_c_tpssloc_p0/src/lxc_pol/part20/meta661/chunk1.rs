//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2477/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2477(t25: f64, t10150: f64, t1074: f64, t11105: f64, t12606: f64, t13493: f64, t1408: f64, t1409: f64, t14675: f64, t1534: f64, t1642: f64, t2249: f64, t2250: f64, t3220: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t45872: f64, t4705: f64, t47655: f64, t47668: f64, t47670: f64, t47672: f64, t47674: f64, t47676: f64, t50785: f64, t606: f64, t607: f64, t9257: f64, t9258: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t50803 = piecewise3(t115, t47655 * t25 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t13493 * t606 + 3.0_f64 / 2.0_f64 * t4324 * t2249 + t1534 * t9257 / 2.0_f64 + t10150 * t1408 / 2.0_f64 + t47668 + t47670 - t47672 - t47674 + t47676, t50785 * t40 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t14675 * t607 + 3.0_f64 / 2.0_f64 * t4705 * t2250 + t1642 * t9258 / 2.0_f64 + t11105 * t1409 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t3220 * t3966 + 3.0_f64 / 2.0_f64 * t1074 * t12606 + t396 * t45872 / 2.0_f64);
    t50803
}
