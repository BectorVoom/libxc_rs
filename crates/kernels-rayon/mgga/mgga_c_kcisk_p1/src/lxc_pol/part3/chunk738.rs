//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 738/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk738(t11430: f64, t4629: f64, t10449: f64, t1876: f64, t1877: f64, t4624: f64, t682: f64, t1824: f64, t7028: f64, t4663: f64, t708: f64, t10664: f64) -> (f64, f64, f64, f64, f64) {
    let t11431 = t4629 * t11430;
    let t11435 = t1876 * t1877 * t10449;
    let t11438 = t682 * t4624;
    let t11439 = t11438 * t1824;
    let t11440 = t7028 * t11439;
    let t11443 = t4663 * t708;
    let t11444 = t11443 * t10664;
    (t11431, t11435, t11439, t11440, t11444)
}
