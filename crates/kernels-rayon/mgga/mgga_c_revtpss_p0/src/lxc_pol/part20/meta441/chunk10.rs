//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1685/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1685(t33: f64, t265: f64, t502: f64, t41211: f64, t44088: f64, t45901: f64, t45903: f64, t45908: f64, t10326: f64, t11095: f64, t1113: f64, t1304: f64, t13196: f64, t2258: f64, t2838: f64, t3351: f64, t3805: f64, t39457: f64, t43744: f64, t504: f64, t57: f64, t606: f64, t895: f64, t9357: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t45911 = piecewise3(t503, t44088 + t45901 + t45903 + t45908, t41211);
    let t45923 = piecewise3(t400, t41211 * t33 / 2.0_f64 + 2.0_f64 * t11095 * t1113 + 3.0_f64 * t2838 * t3351 + 2.0_f64 * t895 * t9357 + t265 * t43744 / 2.0_f64, t45911 * t57 / 2.0_f64 - 2.0_f64 * t13196 * t606 - 3.0_f64 * t3805 * t2258 - 2.0_f64 * t1304 * t10326 - t504 * t39457 / 2.0_f64);
    t45923
}
