//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1074/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1074(t30: f64, t259: f64, t379: f64, t10937: f64, t11219: f64, t11796: f64, t10353: f64, t10947: f64, t10948: f64, t10950: f64, t1288: f64, t1289: f64, t1402: f64, t1490: f64, t1991: f64, t1992: f64, t2445: f64, t2818: f64, t3431: f64, t3735: f64, t381: f64, t4028: f64, t45: f64, t580: f64, t581: f64, t999: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t380 = t259 < t379;
    let t11798 = piecewise3(t380, t11219 + t11796, t10937);
    let t11810 = piecewise3(t120, t10937 * t30 / 2.0_f64 + t3735 * t580 + t1402 * t1991 / 2.0_f64 + t2445 * t1288 / 2.0_f64 + t10947 + t10948 - t10950, t11798 * t45 / 2.0_f64 + t4028 * t581 + t1490 * t1992 / 2.0_f64 + t2818 * t1289 / 2.0_f64 + t999 * t3431 + t381 * t10353 / 2.0_f64);
    t11810
}
