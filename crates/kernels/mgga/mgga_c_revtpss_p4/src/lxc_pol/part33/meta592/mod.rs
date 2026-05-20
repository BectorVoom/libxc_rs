//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2007;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta592<F: Float>(t25894: F, t94394: F, t25945: F, t9285: F, t25944: F, t2482: F, t7262: F, t814: F, t820: F, t844: F, t596: F, t7269: F, t3981: F, t25981: F, t843: F, t2681: F, t1401: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94395, t94407, t94409, t94423, t94429, t94443) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2007::<F>(t25894, t94394, t25945, t9285, t25944, t2482, t7262, t814, t820, t844, t596, t7269);
        let (t94444, t94455, t94459, t94460, t94472, t94473) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2008::<F>(t3981, t94443, t25981, t820, t843, t2681, t7262, t1401, t533, t816, t92993, t7259, t9709);
    (t94395, t94407, t94409, t94423, t94429, t94443, t94444, t94455, t94459, t94460, t94472, t94473)
}
