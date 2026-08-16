//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2010;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta588<F: Float>(t2482: F, t7262: F, t814: F, t820: F, t844: F, t596: F, t7269: F, t3981: F, t25981: F, t843: F, t2681: F, t1401: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F, t1389: F, t3964: F, t92986: F, t7028: F, t9736: F, t9737: F, t26009: F, t9802: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94423, t94429, t94443, t94444, t94455, t94459, t94460) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2010::<F>(t2482, t7262, t814, t820, t844, t596, t7269, t3981, t25981, t843, t2681, t1401);
        let (t94472, t94474, t94477, t94479, t94483) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2011::<F>(t533, t816, t92993, t7259, t9709, t1389, t3964, t92986, t7028, t9736, t9737, t26009, t9802);
    (t94423, t94429, t94443, t94444, t94455, t94459, t94460, t94472, t94474, t94477, t94479, t94483)
}
