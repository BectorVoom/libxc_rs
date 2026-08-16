//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1249/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1249(t2096: f64, t7692: f64, t287: f64, t5913: f64, t17848: f64, t2104: f64, t7641: f64, t17867: f64, t2932: f64, t7607: f64, t7784: f64, t2945: f64, t2947: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21841 = t2096 * t7692;
    let t21843 = t5913 * t287;
    let t21852 = t2104 * t17848 * t7641;
    let t21862 = t2104 * t17867 * t2932;
    let t21863 = 0.28582678745379824648e-3_f64 * t21862;
    let t21867 = t7607 * t7784;
    let t21870 = t2945 * t5939 * t2947;
    (t21841, t21843, t21852, t21863, t21867, t21870)
}
