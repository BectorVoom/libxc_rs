//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 503/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk503(t4319: f64, t4323: f64, t2: f64, t265: f64, t584: f64, t1540: f64, t690: f64) -> (f64, f64, f64) {
    let t4324 = t4319 + t4323;
    let t4331 = t265 * t2;
    let t4332 = t4331 * t584;
    let t4335 = t690 * t1540;
    (t4324, t4332, t4335)
}
