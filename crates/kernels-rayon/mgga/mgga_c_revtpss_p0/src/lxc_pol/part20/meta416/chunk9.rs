//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1556/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1556(t30: f64, t265: f64, t393: f64, t41211: f64, t41477: f64, t41574: f64, t41943: f64, t43720: f64, t10326: f64, t1106: f64, t11095: f64, t12201: f64, t2257: f64, t2258: f64, t2838: f64, t3340: f64, t39456: f64, t39457: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, t9344: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t43723 = piecewise3(t394, t41477 + t41574 + t41943 + t43720, t41211);
    let t43735 = piecewise3(t120, t41211 * t30 / 2.0_f64 + 2.0_f64 * t11095 * t605 + 3.0_f64 * t2838 * t2257 + 2.0_f64 * t895 * t9344 + t265 * t39456 / 2.0_f64, t43723 * t45 / 2.0_f64 + 2.0_f64 * t12201 * t606 + 3.0_f64 * t3340 * t2258 + 2.0_f64 * t1106 * t10326 + t395 * t39457 / 2.0_f64);
    t43735
}
