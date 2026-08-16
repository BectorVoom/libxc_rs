//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 909/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk909(t45423: f64, t6066: f64, t6111: f64, t10914: f64, t10915: f64, t326: f64, t45369: f64, t825: f64, t13588: f64, t549: f64, t11757: f64, t9823: f64) -> (f64, f64, f64, f64, f64) {
    let t45426 = 0.42900587942220512003e1_f64 * t6111 * t6066 * t45423;
    let t45429 = 0.21450293971110256001e1_f64 * t10914 * t10915 * t45423;
    let t45432 = 0.18404604457881959845e2_f64 * t825 * t326 * t45369;
    let t45437 = t6111 * t549 * t13588;
    let t45438 = 0.59584149919750711116e-1_f64 * t45437;
    let t45440 = 0.35750489951850426669e0_f64 * t9823 * t11757;
    (t45426, t45429, t45432, t45438, t45440)
}
