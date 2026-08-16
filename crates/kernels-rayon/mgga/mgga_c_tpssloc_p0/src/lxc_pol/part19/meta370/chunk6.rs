//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1377/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1377(t25: f64, t265: f64, t394: f64, t41606: f64, t42274: f64, t43627: f64, t43641: f64, t43642: f64, t10150: f64, t1074: f64, t11105: f64, t2249: f64, t2250: f64, t2756: f64, t3220: f64, t39109: f64, t39110: f64, t396: f64, t40: f64, t606: f64, t607: f64, t873: f64, t9257: f64, t9258: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t43645 = piecewise3(t395, t42274 + t43627 + t43641 + t43642, t41606);
    let t43657 = piecewise3(t115, t41606 * t25 / 2.0_f64 + 2.0_f64 * t10150 * t606 + 3.0_f64 * t2756 * t2249 + 2.0_f64 * t873 * t9257 + t265 * t39109 / 2.0_f64, t43645 * t40 / 2.0_f64 + 2.0_f64 * t11105 * t607 + 3.0_f64 * t3220 * t2250 + 2.0_f64 * t1074 * t9258 + t396 * t39110 / 2.0_f64);
    t43657
}
