//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 917/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk917(t1835: f64, t254: f64, t225: f64, t28053: f64, t10143: f64, t1408: f64, t214: f64, t5631: f64, t28437: f64, t28442: f64, t1520: f64, t1902: f64, t5611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t97740 = t1835 * t254;
    let t97756 = t28053 * t225;
    let t98064 = t10143 * t1408;
    let t98133 = t214 * t5631;
    let t98166 = t28437 * t225;
    let t98239 = t28442 * t225;
    let t98279 = t1520 * t254;
    let t98494 = t1902 * t5611;
    (t97740, t97756, t98064, t98133, t98166, t98239, t98279, t98494)
}
