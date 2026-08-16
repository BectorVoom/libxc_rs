//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1890;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta516(t3: f64, t3966: f64, t1484: f64, t1530: f64, t16596: f64, t1877: f64, t1915: f64, t193: f64, t202: f64, t23290: f64, t23295: f64, t2522: f64, t25353: f64, t25358: f64, t25365: f64, t25374: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t6666: f64, t6670: f64, t7541: f64, t776: f64, t868: f64, t870: f64, t28: f64, t23788: f64, t1081: f64, t10143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25588, t25882) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1890(t3, t3966, t1484, t1530, t16596, t1877, t1915, t193, t202, t23290, t23295, t2522, t25353, t25358, t25365, t25374, t4119, t4255, t4303, t4314, t6666, t6670, t7541, t776, t868, t870);
        let (t25891, t25892, t25898, t25901, t25905, t25921, t25927) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1891(t28, t870, t4255, t16596, t23788, t1081, t1484, t4119, t25365, t10143);
    (t25588, t25882, t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
