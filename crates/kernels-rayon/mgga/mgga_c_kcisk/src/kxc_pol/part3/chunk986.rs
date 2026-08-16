//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 986/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk986(t14535: f64, t493: f64, t1483: f64, t4309: f64, t1492: f64, t4174: f64, t486: f64, t1497: f64, t4297: f64, t13329: f64, t492: f64, t13331: f64, t499: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14536 = t493 * t14535;
    let t14538 = t1483 * t4309;
    let t14540 = t1492 * t4174;
    let t14541 = t486 * t14540;
    let t14543 = t4297 * t1497;
    let t14545 = t13329 * t492;
    let t14546 = t499 * t13331;
    (t14536, t14538, t14541, t14543, t14545, t14546)
}
