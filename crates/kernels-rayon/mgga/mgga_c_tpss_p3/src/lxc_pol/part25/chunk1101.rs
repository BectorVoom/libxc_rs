//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1101/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1101(t30: f64, t259: f64, t379: f64, t14432: f64, t14689: f64, t15206: f64, t1288: f64, t1289: f64, t13334: f64, t13335: f64, t14440: f64, t1490: f64, t3431: f64, t3735: f64, t381: f64, t4028: f64, t45: f64, t4578: f64, t4579: f64, t4818: f64, t5048: f64, t580: f64, t581: f64, t826: f64, t999: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t15208 = piecewise3(t380, t14689 + t15206, t14432);
    let t15220 = piecewise3(t120, t14432 * t30 / 2.0_f64 + t4818 * t580 / 2.0_f64 + t3735 * t1288 + t14440 + t826 * t4578 / 2.0_f64 + t259 * t13334 / 2.0_f64, t15208 * t45 / 2.0_f64 + t5048 * t581 / 2.0_f64 + t4028 * t1289 + t1490 * t3431 + t999 * t4579 / 2.0_f64 + t381 * t13335 / 2.0_f64);
    t15220
}
