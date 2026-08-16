//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1027/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1027(t2633: f64, t4180: f64, t4181: f64, t13029: f64, t225: f64, t237: f64, t2697: f64, t4261: f64, t12971: f64, t820: f64, t847: f64, t9645: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13333 = t4180 * t4181 * t2633;
    let t13336 = t13029 * t225;
    let t13337 = t13336 * t237;
    let t13345 = 7.0_f64 / 576.0_f64 * t2697 * t4261;
    let t13347 = t847 * t820 * t12971;
    let t13350 = t9645 * t820;
    (t13333, t13336, t13337, t13345, t13347, t13350)
}
