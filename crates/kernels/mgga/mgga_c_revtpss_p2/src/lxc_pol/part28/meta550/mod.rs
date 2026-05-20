//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta550<F: Float>(t1455: F, t7337: F, t2045: F, t4153: F, t10301: F, t607: F, t1927: F, t2248: F, t1926: F, t25163: F, t6973: F, t644: F, t6977: F, t2315: F, t2247: F, t2259: F, t2269: F, t48: F, t2275: F, t613: F, t10355: F, t43: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92559, t92563, t92565, t92570, t92573, t92576) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1999::<F>(t1455, t7337, t2045, t4153, t10301, t607, t1927, t2248, t1926, t25163, t6973, t644, t6977);
        let (t92577, t92585, t92588, t92597, t92600, t92605) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2000::<F>(t1926, t92576, t1927, t2315, t2247, t2259, t2269, t48, t2275, t613, t10355, t43);
    (t92559, t92563, t92565, t92570, t92573, t92577, t92585, t92588, t92597, t92600, t92605)
}
