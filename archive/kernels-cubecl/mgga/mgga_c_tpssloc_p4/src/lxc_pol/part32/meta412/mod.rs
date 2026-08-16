//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1589;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1590;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta412<F: Float>(t136: F, t18499: F, t18215: F, t3297: F, t6014: F, t699: F, t1113: F, t18221: F, t18225: F, t6017: F, t18232: F, t18237: F, t18241: F, t11211: F, t11487: F, t14766: F, t15347: F, t15348: F, t15349: F, t18494: F, t18497: F, t457: F, t460: F, t974: F, t135: F, t6146: F, t1174: F, t6140: F, t11558: F, t15341: F, t15364: F, t15366: F, t15374: F, t15376: F, t18475: F, t18484: F, t18489: F, t3447: F, t4905: F, t4909: F, t4920: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18500, t18503, t18505, t18508, t18510, t18512, t18515, t18517) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1589::<F>(t136, t18499, t18215, t3297, t6014, t699, t1113, t18221, t18225, t6017, t18232, t18237);
        let (t18518, t18521, t18523) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1590::<F>(t136, t18517, t1113, t18241, t11211, t11487, t14766, t15347, t15348, t15349, t18494, t18497, t18500, t18503, t18505, t18508, t18510, t18512, t18515);
        let (t18525, t18535) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1591::<F>(t18523, t457, t460, t974, t135, t6146, t1174, t6140, t11558, t15341, t15364, t15366, t15374, t15376, t18475, t18484, t18489, t3447, t4905, t4909, t4920);
    (t18500, t18503, t18505, t18508, t18510, t18512, t18515, t18518, t18521, t18523, t18525, t18535)
}
