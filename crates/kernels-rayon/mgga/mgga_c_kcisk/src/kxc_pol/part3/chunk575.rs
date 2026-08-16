//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 575/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk575(t1896: f64, t4811: f64, t1901: f64, t1862: f64, t1871: f64, t1895: f64, t1869: f64, t1691: f64, t670: f64, t604: f64, t1790: f64, t667: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4812 = t4811 * t1896;
    let t4814 = t4811 * t1901;
    let t4816 = t1862 * t1871;
    let t4817 = t4816 * sigma2;
    let t4818 = t4817 * t1895;
    let t4819 = t1869 * t4818;
    let t4822 = 1.0_f64 / t1691 / t670;
    let t4823 = t604 * t4822;
    let t4824 = t1790 * t1790;
    let t4825 = t667 * t667;
    (t4812, t4814, t4816, t4817, t4818, t4819, t4822, t4823, t4824, t4825)
}
