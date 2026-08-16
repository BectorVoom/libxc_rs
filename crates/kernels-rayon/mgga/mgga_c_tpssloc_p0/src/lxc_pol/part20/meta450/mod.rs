//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1900;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta450(t1117: f64, t4782: f64, t3264: f64, t1671: f64, t3307: f64, t3265: f64, t4785: f64, t11190: f64, t3315: f64, t4781: f64, t3313: f64, t11277: f64, t1670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15051, t15053, t15054, t15056, t15057, t15059, t15060, t15061, t15063, t15064, t15066, t15067) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1900(t1117, t4782, t3264, t1671, t3307, t3265, t4785, t11190, t3315, t4781, t3313, t11277, t1670);
    (t15051, t15053, t15054, t15056, t15057, t15059, t15060, t15061, t15063, t15064, t15066, t15067)
}
