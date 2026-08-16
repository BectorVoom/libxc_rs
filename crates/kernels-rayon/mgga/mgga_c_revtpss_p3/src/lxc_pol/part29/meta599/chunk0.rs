//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2039/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2039(t101761: f64, t103720: f64, t103868: f64, t118: f64, t1310: f64, t13426: f64, t13514: f64, t18163: f64, t1843: f64, t2014: f64, t2089: f64, t2322: f64, t2372: f64, t25177: f64, t26210: f64, t26396: f64, t28586: f64, t28653: f64, t28683: f64, t28711: f64, t28737: f64, t28750: f64, t28926: f64, t4151: f64, t4254: f64, t508: f64, t5517: f64, t651: f64, t670: f64, t7315: f64, t7357: f64, t7378: f64, t7488: f64, t7732: f64, t7900: f64, t7988: f64, t8075: f64, t8108: f64, t95464: f64, t98564: f64) -> f64 {
    let t103873 = t8075 * t4151 - 2.0_f64 * t651 * t508 * t101761 - t26210 * t1843 - 2.0_f64 * t7357 * t5517 - 2.0_f64 * t2014 * t28926 * t7315 - 2.0_f64 * t18163 * t7988 - 4.0_f64 * t4254 * t28750 - 2.0_f64 * t28653 * t2372 - 4.0_f64 * t13426 * t7378 - 2.0_f64 * t651 * t2089 * t13514 - 4.0_f64 * t2322 * t28737 - 4.0_f64 * t651 * t28586 * t670 - 4.0_f64 * t7732 * t26396 + 3.0_f64 * t2014 * t95464 * t7900 + 2.0_f64 * t2014 * t8108 * t25177 + 3.0_f64 * t2014 * t7488 * t98564 - 4.0_f64 * t651 * t1310 * t28683 - t118 * (t103720 + t103868) - 4.0_f64 * t2322 * t28711;
    t103873
}
