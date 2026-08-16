//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1609;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta394(t3378: f64, t4882: f64, t1164: f64, t3411: f64, t4879: f64, t11433: f64, t3396: f64, t4874: f64, t11424: f64, t4745: f64, t11185: f64, t4786: f64, t1117: f64, t4782: f64, t3264: f64, t1671: f64, t3307: f64, t3265: f64, t4785: f64, t11190: f64, t3315: f64, t4781: f64, t3313: f64, t11277: f64, t1670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15038, t15040, t15043, t15046, t15048, t15050) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1609(t3378, t4882, t1164, t3411, t4879, t11433, t3396, t4874, t11424, t4745, t11185, t4786);
        let (t15053, t15056, t15059, t15063, t15066, t15067) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1610(t1117, t4782, t3264, t1671, t3307, t3265, t4785, t11190, t3315, t4781, t3313, t11277, t1670);
    (t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15056, t15059, t15063, t15066, t15067)
}
