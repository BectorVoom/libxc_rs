//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 982/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk982(t24574: f64, t7288: f64, t225: f64, t7306: f64, t3640: f64, t7394: f64, t11947: f64, t2157: f64, t111: f64, t7263: f64) -> (f64, f64, f64, f64, f64) {
    let t24891 = t24574 * t7288;
    let t24893 = t7306 * t225;
    let t24905 = t7394 * t3640;
    let t24909 = t2157 * t11947;
    let t24932 = t7263 * t111;
    (t24891, t24893, t24905, t24909, t24932)
}
