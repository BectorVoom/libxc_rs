//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1358/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1358<F: Float>(t132: F, t24480: F, t11003: F, t2601: F, t4323: F, t6996: F, t1005: F, t11086: F, t11089: F, t11090: F, t11095: F, t11098: F, t11101: F, t11104: F, t11108: F, t11139: F, t11140: F, t21507: F, t21679: F, t21715: F, t21726: F, t25561: F, t25680: F, t25737: F, t2578: F, t25806: F, t25819: F, t2593: F, t2599: F, t3583: F, t4310: F, t6993: F, t7104: F, t7140: F, t7159: F, t9086: F, t9090: F, t9195: F, t9210: F, t986: F, zeta_threshold: F) -> (F, F) {
    let t133 = t132 <= zeta_threshold;
    let t29538 = piecewise3::<f64>(t133, F::new(0.0), -t24480);
    let t29559 = t11003 * t2601;
    let t29566 = t4323 * t6996;
    let t29598 = F::new(0.8276162067083744048e4) * t25806 * t25680 * t986 + F::new(0.41016075432865626631e4) * t25819 * t25737 * t1005 + F::new(0.34631718211362927518e2) * t7104 * t11140 + F::new(0.34631718211362927518e2) * t2599 * t29559 * t1005 + F::new(0.17315859105681463759e2) * t2599 * t11139 * t2593 + F::new(0.10254018858216406658e4) * t6993 * t29566 * t2578 + F::new(0.69263436422725855036e2) * t7104 * t11086 + F::new(0.34631718211362927518e2) * t2599 * t3583 * t9195 + F::new(0.20508037716432813316e4) * t21726 * t11090 + F::new(0.10254018858216406658e4) * t6993 * t11089 * t2593 + F::new(0.91082604192152556044e5) * t21715 * t4310 * t21507 * t2578 + F::new(0.64327917994770140268e2) * t9210 * t9086 + F::new(0.4138081033541872024e4) * t25561 * t9090 + F::new(12.0) * t7159 * t11095 - F::new(8.0) * t7140 * t11098 - F::new(0.38596750796862084162e3) * t21679 * t11101 - F::new(4.0) * t7140 * t11104 + F::new(0.64327917994770140268e2) * t7159 * t11108;
    (t29538, t29598)
}
