//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1146/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1146(t59: f64, t9971: f64, t240: f64, t812: f64, t9978: f64, t6613: f64, t9612: f64, t831: f64, t23040: f64, t2617: f64, t232: f64, t25119: f64, t47072: f64, t815: f64) -> (f64, f64, f64, f64) {
    let t81816 = t9971 * t59;
    let t81818 = t812 * t81816 * t240;
    let t81819 = t81818 * t9978;
    let t81821 = t9612 * t6613;
    let t81822 = t81821 * t831;
    let t81824 = t2617 * t23040;
    let t81825 = t81824 * t831;
    let t81829 = t25119 * t815 * t47072 * t232;
    (t81819, t81822, t81825, t81829)
}
