//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 903/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk903(t6562: f64, t82133: f64, t8335: f64, t23168: f64, t30664: f64, t1880: f64, t214: f64, t225: f64, t23150: f64, t258: f64, t30643: f64, t6547: f64) -> (f64, f64, f64, f64) {
    let t112741 = t6562 * t82133 * t8335;
    let t112742 = 0.16449340668482264365e-1_f64 * t112741;
    let t112743 = t23168 * t30664;
    let t112744 = 0.15352717957250113407e0_f64 * t112743;
    let t112759 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t23150 * t225 * t258;
    let t112760 = t6547 * t30643;
    (t112742, t112744, t112759, t112760)
}
