//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2531;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta702<F: Float>(t13823: F, t2960: F, t13816: F, t2970: F, t973: F, t13828: F, t10224: F, t4522: F, t13895: F, t1599: F, t2402: F, t13908: F, t10263: F, t4528: F, t12606: F, t2989: F, t2986: F, t344: F, t43052: F, t4343: F, t2978: F, t4338: F, t697: F, t43053: F, t4514: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t48297, t48302, t48317, t48320, t48328, t48336, t48338) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2531::<F>(t13823, t2960, t13816, t2970, t973, t13828, t10224, t4522, t13895, t1599, t2402, t13908);
        let (t48342, t48357, t48373, t48378, t48381) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2532::<F>(t10263, t4528, t12606, t2989, t2986, t344, t43052, t4343, t2978, t4338, t697, t43053, t4514);
    (t48297, t48302, t48317, t48320, t48328, t48336, t48338, t48342, t48357, t48373, t48378, t48381)
}
