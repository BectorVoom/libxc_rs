//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2542;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta674<F: Float>(t11269: F, t1671: F, t3264: F, t11191: F, t15067: F, t43969: F, t15060: F, t3307: F, t3313: F, t11277: F, t4781: F, t11275: F, t3265: F, t4785: F, t1670: F, t44075: F, t44077: F, t11403: F, t14838: F, t11407: F, t14850: F, t44159: F, t4745: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t51453, t51456, t51459, t51463) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2542::<F>(t11269, t1671, t3264, t11191, t15067, t43969, t15060, t3307, t3313, t11277, t4781, t11275, t3265);
        let (t51466, t51470, t51472, t51474, t51476) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2543::<F>(t11269, t3313, t4785, t11191, t1670, t44075, t44077, t11403, t14838, t11407, t14850, t44159, t4745);
    (t51453, t51456, t51459, t51463, t51466, t51470, t51472, t51474, t51476)
}
