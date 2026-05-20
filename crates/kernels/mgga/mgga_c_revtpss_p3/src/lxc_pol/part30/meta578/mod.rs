//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2029;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta578<F: Float>(t94471: F, t7259: F, t9709: F, t1389: F, t3964: F, t92986: F, t7028: F, t9736: F, t9737: F, t26009: F, t9802: F, t26004: F, t3961: F, t64: F, t9990: F, t2482: F, t596: F, t7262: F, t4021: F, t25986: F, t2661: F, t9980: F, t26024: F, t3926: F, t4059: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94472, t94474, t94477, t94479, t94484, t94485) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2029::<F>(t94471, t7259, t9709, t1389, t3964, t92986, t7028, t9736, t9737, t26009, t9802, t26004, t3961);
        let (t94491, t94497, t94498, t94501, t94503, t94505) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2030::<F>(t64, t9990, t2482, t596, t7262, t4021, t25986, t2661, t9980, t26024, t3926, t4059);
    (t94472, t94474, t94477, t94479, t94484, t94485, t94491, t94497, t94498, t94501, t94503, t94505)
}
