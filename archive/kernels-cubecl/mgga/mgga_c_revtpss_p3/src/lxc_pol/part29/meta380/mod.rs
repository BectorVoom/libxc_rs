//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1360;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta380<F: Float>(t136: F, t1903: F, t2457: F, t9674: F, t10175: F, t5722: F, t122: F, t5721: F, t3916: F, t9680: F, t1437: F, t1882: F, t2482: F, t4104: F, t10073: F, t5737: F, t1419: F, t4086: F, t543: F, t2782: F, t555: F, t5658: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14104, t14105, t14108, t14109, t14110, t14111, t14113) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1360::<F>(t136, t1903, t2457, t9674, t10175, t5722, t122, t5721, t3916, t9680, t1437, t1882);
        let (t14116, t14120, t14122, t14126, t14127) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1361::<F>(t14113, t2482, t4104, t10073, t5737, t1419, t1882, t4086, t543, t2782, t555, t5658);
    (t14104, t14105, t14108, t14109, t14110, t14111, t14116, t14120, t14122, t14126, t14127)
}
