//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 616/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk616(t25: f64, t265: f64, t394: f64, t2165: f64, t671: f64, t6834: f64, t2116: f64, t40: f64, t607: f64, t6678: f64, t1170: f64, t2123: f64, t2121: f64, t2127: f64, t6686: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7271 = t2165 * t671;
    let t7274 = piecewise3(t395, 0.0_f64, t6834);
    let t7279 = piecewise3(t115, t6678, t2116 * t607 / 2.0_f64 + t7274 * t40 / 2.0_f64);
    let t7280 = t1170 * t2123;
    let t7282 = 0.27415567780803773942e-2_f64 * t2121 * t7280;
    let t7283 = t2127 * t6686;
    (t7271, t7274, t7279, t7282, t7283)
}
