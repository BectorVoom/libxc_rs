//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1017/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1017(t33: f64, t1469: f64, t33896: f64, t35008: f64, t57: f64, t8960: f64, t34393: f64, t118: f64, t1502: f64, t1843: f64, t2127: f64, t2163: f64, t33664: f64, t33666: f64, t33669: f64, t33916: f64, t33920: f64, t33977: f64, t34429: f64, t34434: f64, t34444: f64, t34447: f64, t34449: f64, t34464: f64, t34874: f64, t508: f64, t8152: f64, t8233: f64, t8463: f64, t8917: f64, t8964: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t35013 = piecewise3(t400, t33896, -t8960 * t1469 / 2.0_f64 + t35008 * t57 / 2.0_f64);
    let t35014 = t34393 + t35013;
    let t35017 = -t118 * t35014 - t1502 * t8964 - t1843 * t8917 - 2.0_f64 * t2127 * t8233 - 2.0_f64 * t2163 * t8152 - t34874 * t508 - t33664 - t33666 + t33669 - t33916 + t33920 + t33977 - 4.0_f64 * t34429 - 4.0_f64 * t34434 - 4.0_f64 * t34444 - 4.0_f64 * t34447 - 4.0_f64 * t34449 + 6.0_f64 * t34464 - t8463;
    (t35014, t35017)
}
