//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1358/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1358(t132: f64, t24480: f64, t11003: f64, t2601: f64, t4323: f64, t6996: f64, t1005: f64, t11086: f64, t11089: f64, t11090: f64, t11095: f64, t11098: f64, t11101: f64, t11104: f64, t11108: f64, t11139: f64, t11140: f64, t21507: f64, t21679: f64, t21715: f64, t21726: f64, t25561: f64, t25680: f64, t25737: f64, t2578: f64, t25806: f64, t25819: f64, t2593: f64, t2599: f64, t3583: f64, t4310: f64, t6993: f64, t7104: f64, t7140: f64, t7159: f64, t9086: f64, t9090: f64, t9195: f64, t9210: f64, t986: f64, zeta_threshold: f64) -> (f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t29538 = piecewise3(t133, 0.0_f64, -t24480);
    let t29559 = t11003 * t2601;
    let t29566 = t4323 * t6996;
    let t29598 = 0.8276162067083744048e4_f64 * t25806 * t25680 * t986 + 0.41016075432865626631e4_f64 * t25819 * t25737 * t1005 + 0.34631718211362927518e2_f64 * t7104 * t11140 + 0.34631718211362927518e2_f64 * t2599 * t29559 * t1005 + 0.17315859105681463759e2_f64 * t2599 * t11139 * t2593 + 0.10254018858216406658e4_f64 * t6993 * t29566 * t2578 + 0.69263436422725855036e2_f64 * t7104 * t11086 + 0.34631718211362927518e2_f64 * t2599 * t3583 * t9195 + 0.20508037716432813316e4_f64 * t21726 * t11090 + 0.10254018858216406658e4_f64 * t6993 * t11089 * t2593 + 0.91082604192152556044e5_f64 * t21715 * t4310 * t21507 * t2578 + 0.64327917994770140268e2_f64 * t9210 * t9086 + 0.4138081033541872024e4_f64 * t25561 * t9090 + 12.0_f64 * t7159 * t11095 - 8.0_f64 * t7140 * t11098 - 0.38596750796862084162e3_f64 * t21679 * t11101 - 4.0_f64 * t7140 * t11104 + 0.64327917994770140268e2_f64 * t7159 * t11108;
    (t29538, t29598)
}
