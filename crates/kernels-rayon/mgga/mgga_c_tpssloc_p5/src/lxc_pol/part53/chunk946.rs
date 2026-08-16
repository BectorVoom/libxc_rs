//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 946/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk946(t214: f64, t7084: f64, t31329: f64, t6547: f64, t23030: f64, t31319: f64, t23168: f64, t31367: f64, t114790: f64, t23164: f64, t6555: f64, t2047: f64, t212: f64, t23171: f64, t6554: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114866 = t214 * t7084;
    let t114882 = t6547 * t31329;
    let t114891 = t23030 * t31319;
    let t114900 = t23168 * t31367;
    let t114916 = t23164 * t114790 * t6555;
    let t114932 = t23171 * t212 * t2047 * t6554;
    (t114866, t114882, t114891, t114900, t114916, t114932)
}
