//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 638/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk638(t41: f64, t4143: f64, t1849: f64, t719: f64, t4594: f64, t704: f64, t1336: f64, t140: f64, t4597: f64, t1683: f64, t4790: f64, t4595: f64, t708: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6443 = t41 * t4143;
    let t6666 = t719 * t1849;
    let t6672 = t4594 * t704;
    let t6674 = t140 * t1336 * t6672;
    let t6675 = t719 * t4597;
    let t6880 = t4790 * t1683;
    let t7000 = t4595 * t708;
    (t6443, t6666, t6674, t6675, t6880, t7000)
}
