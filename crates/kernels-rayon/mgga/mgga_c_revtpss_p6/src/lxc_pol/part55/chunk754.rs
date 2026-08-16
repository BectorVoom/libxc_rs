//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 754/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk754(t1711: f64, t1940: f64, t1963: f64, t2403: f64, t33: f64, t7091: f64, t7783: f64, t7863: f64, t7869: f64, t1936: f64, t4248: f64, t1518: f64, t93: f64) -> (f64, f64, f64) {
    let t7876 = 3.0_f64 / 2.0_f64 * t2403 * t7863 + t1940 * t7783 * t33 / 2.0_f64 - t1940 * t7091 * t7869 / 2.0_f64 + t1940 * t1963 * t1711 / 2.0_f64;
    let t7888 = 2.0_f64 * t4248 * t1936;
    let t7889 = t93 * t1518;
    (t7876, t7888, t7889)
}
