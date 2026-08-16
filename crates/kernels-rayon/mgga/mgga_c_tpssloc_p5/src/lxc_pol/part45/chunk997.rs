//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 997/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk997(t25: f64, t265: f64, t394: f64, t115099: f64, t114991: f64, t115040: f64, t2250: f64, t31478: f64, t40: f64, t607: f64, t8580: f64, t1081: f64, t113751: f64, t113764: f64, t113772: f64, t114977: f64, t115000: f64, t115009: f64, t115012: f64, t1877: f64, t23781: f64, t23788: f64, t23789: f64, t23810: f64, t24191: f64, t24339: f64, t26563: f64, t26756: f64, t31430: f64, t31434: f64, t31441: f64, t31448: f64, t31504: f64, t4314: f64, t7114: f64, t83555: f64, t84791: f64, t8566: f64, t8586: f64, t89849: f64, t89953: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t115100 = piecewise3(t395, 0.0_f64, t115099);
    let t115107 = piecewise3(t115, t114991 + t115040, t115100 * t40 / 2.0_f64 + t31478 * t607 + t8580 * t2250 / 2.0_f64);
    let t115143 = -3.0_f64 / 2.0_f64 * t24191 * t23788 * t115000 + 3.0_f64 * t4314 * t8566 * t23781 - t1877 * t84791 * t8586 / 2.0_f64 - t1877 * t31434 * t23810 - 3.0_f64 * t24191 * t83555 * t31441 - 3.0_f64 * t115009 * t23789 - 3.0_f64 * t26756 * t89953 * t115012 - t1877 * t7114 * t113751 + 2.0_f64 * t26756 * t113764 + 2.0_f64 * t26756 * t89849 * t31448 - 3.0_f64 * t26563 * t23788 * t114977 - t1877 * t24339 * t31504 - 3.0_f64 * t24191 * t113772 + t1877 * t31430 * t1081;
    (t115107, t115143)
}
