//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 737/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk737(t10463: f64, t708: f64, t10441: f64, t11417: f64, t3521: f64, t4616: f64, t4652: f64, t682: f64, t1824: f64, t4629: f64, t4630: f64, t4684: f64) -> (f64, f64, f64, f64, f64) {
    let t11418 = t708 * t10463;
    let t11420 = t11417 * t11418 * t10441;
    let t11423 = t3521 * t4616;
    let t11425 = t682 * t4652;
    let t11426 = t11425 * t1824;
    let t11427 = t4629 * t11426;
    let t11430 = t4630 * t4684;
    (t11420, t11423, t11426, t11427, t11430)
}
