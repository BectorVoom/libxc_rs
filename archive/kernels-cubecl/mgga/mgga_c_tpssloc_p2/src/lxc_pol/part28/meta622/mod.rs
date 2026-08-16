//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1944;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta622<F: Float>(t1361: F, t16153: F, t26288: F, t1339: F, t16206: F, t6936: F, t1825: F, t22827: F, t3719: F, t1307: F, t7708: F, t80840: F, t90787: F, t26245: F, t80783: F, t22897: F, t6925: F, t12369: F, t1351: F, t26243: F, t26302: F, t80958: F, t22779: F, t26323: F, t1336: F, t242: F, t80901: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91333, t91336, t91340, t91344) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1944::<F>(t1361, t16153, t26288, t1339, t16206, t6936, t1825, t22827, t3719, t1307, t7708, t80840, t90787);
        let (t91346, t91354, t91356, t91358, t91361) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1945::<F>(t26245, t80783, t22897, t6925, t12369, t1351, t26243, t26302, t80958, t22779, t26323, t1336, t242, t80901);
    (t91333, t91336, t91340, t91344, t91346, t91354, t91356, t91358, t91361)
}
