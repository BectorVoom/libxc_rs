//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 950/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk950(t6547: f64, t6568: f64, t23030: f64, t6563: f64, t6567: f64, t794: f64, t6562: f64, t1883: f64, t23012: f64, t213: f64, t225: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23249 = t6547 * t6568;
    let t23250 = 0.38381794893125283518e-1_f64 * t23249;
    let t23251 = t23030 * t6563;
    let t23252 = 0.26044789391763585244e-1_f64 * t23251;
    let t23253 = t794 * t6567;
    let t23254 = t6562 * t23253;
    let t23261 = t23012 * t1883;
    let t23262 = 0.63969658155208805863e-1_f64 * t23261;
    let t23270 = t213 * t252 * t225;
    (t23249, t23250, t23251, t23252, t23254, t23261, t23262, t23270)
}
