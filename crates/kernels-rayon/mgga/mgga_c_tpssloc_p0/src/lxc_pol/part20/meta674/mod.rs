//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2542;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta674(t11269: f64, t1671: f64, t3264: f64, t11191: f64, t15067: f64, t43969: f64, t15060: f64, t3307: f64, t3313: f64, t11277: f64, t4781: f64, t11275: f64, t3265: f64, t4785: f64, t1670: f64, t44075: f64, t44077: f64, t11403: f64, t14838: f64, t11407: f64, t14850: f64, t44159: f64, t4745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51453, t51456, t51459, t51463) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2542(t11269, t1671, t3264, t11191, t15067, t43969, t15060, t3307, t3313, t11277, t4781, t11275, t3265);
        let (t51466, t51470, t51472, t51474, t51476) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2543(t11269, t3313, t4785, t11191, t1670, t44075, t44077, t11403, t14838, t11407, t14850, t44159, t4745);
    (t51453, t51456, t51459, t51463, t51466, t51470, t51472, t51474, t51476)
}
