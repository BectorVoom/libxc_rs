//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1851/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1851(t4680: f64, t4684: f64, t11060: f64, t3040: f64, t1629: f64, t4673: f64, t1049: f64, t4649: f64, t1060: f64, t11066: f64, t1615: f64, t3166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14574 = t4680 * t4684;
    let t14577 = t11060 * t3040;
    let t14578 = t1629 * t14577;
    let t14581 = t4680 * t4673;
    let t14586 = t1049 * t4649;
    let t14587 = t14586 * t1060;
    let t14590 = t11066 * t3040;
    let t14591 = t1629 * t14590;
    let t14595 = t3166 * t1615;
    (t14574, t14577, t14578, t14581, t14586, t14587, t14590, t14591, t14595)
}
