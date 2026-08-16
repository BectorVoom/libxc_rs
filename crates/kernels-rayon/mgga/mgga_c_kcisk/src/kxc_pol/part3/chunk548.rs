//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 548/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk548(t4597: f64, t708: f64, t3290: f64, t4595: f64, t1797: f64, t574: f64, t1636: f64, t1648: f64, t1876: f64, t682: f64, t1824: f64, t1849: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4598 = t708 * t4597;
    let t4600 = t4595 * t4598 * t3290;
    let t4603 = t1797 * t574;
    let t4604 = t4603 * t708;
    let t4605 = t1636 * t1648;
    let t4606 = t4604 * t4605;
    let t4609 = t1876 * t682;
    let t4610 = t1636 * t1824;
    let t4611 = t4609 * t4610;
    let t4614 = t708 * t1849;
    (t4598, t4600, t4604, t4606, t4609, t4611, t4614)
}
