//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1315/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1315(t28192: f64, t80727: f64, t22892: f64, t7691: f64, t90544: f64, t28200: f64, t6883: f64, t225: f64, t28053: f64, t28237: f64, t532: f64, t2752: f64, t28447: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97664 = t80727 * t28192;
    let t97732 = t22892 * t90544 * t7691;
    let t97750 = t6883 * t28200;
    let t97756 = t28053 * t225;
    let t97817 = t532 * t28237;
    let t98054 = t28447 * t2752;
    (t97664, t97732, t97750, t97756, t97817, t98054)
}
