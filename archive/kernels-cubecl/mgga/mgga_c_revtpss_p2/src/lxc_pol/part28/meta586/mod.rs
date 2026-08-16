//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2052;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta586<F: Float>(t3981: F, t94443: F, t25986: F, t2661: F, t9930: F, t25981: F, t820: F, t843: F, t4006: F, t2681: F, t7262: F, t1401: F, t25997: F, t9905: F, t533: F, t816: F, t92993: F, t7259: F, t9709: F, t1389: F, t3964: F, t92986: F, t7028: F, t9736: F, t9737: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t94444, t94449, t94456, t94459, t94460) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2052::<F>(t3981, t94443, t25986, t2661, t9930, t25981, t820, t843, t4006, t2681, t7262, t1401);
        let (t94468, t94472, t94474, t94477, t94479) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2053::<F>(t25997, t9905, t533, t816, t92993, t7259, t9709, t1389, t3964, t92986, t7028, t9736, t9737);
    (t94444, t94449, t94456, t94459, t94460, t94468, t94472, t94474, t94477, t94479)
}
