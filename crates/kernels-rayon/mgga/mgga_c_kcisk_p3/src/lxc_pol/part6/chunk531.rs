//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 531/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk531(t2514: f64, t3521: f64, t4595: f64, t708: f64, t1876: f64, t1417: f64, t2518: f64, t1646: f64, t673: f64, t2372: f64, t682: f64, t2522: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6998 = t3521 * t2514;
    let t7000 = t4595 * t708;
    let t7012 = t1876 * t708;
    let t7020 = t1417 * t2518;
    let t7028 = t673 * t1646;
    let t7029 = t708 * t2372;
    let t7034 = t682 * t2372;
    let t7043 = t1417 * t2522;
    (t6998, t7000, t7012, t7020, t7028, t7029, t7034, t7043)
}
