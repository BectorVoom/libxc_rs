//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2054;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta587<F: Float>(t26009: F, t9802: F, t26004: F, t3961: F, t64: F, t9990: F, t2482: F, t596: F, t7262: F, t4021: F, t25986: F, t2661: F, t9980: F, t26024: F, t3926: F, t4059: F, t25981: F, t27: F, t10003: F, t25997: F, t9970: F, t550: F, t7021: F, t3946: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94484, t94485, t94491, t94497, t94498, t94501) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2054::<F>(t26009, t9802, t26004, t3961, t64, t9990, t2482, t596, t7262, t4021, t25986, t2661, t9980);
        let (t94503, t94505, t94509, t94511, t94514) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2055::<F>(t26024, t3926, t4059, t2482, t25981, t27, t10003, t25997, t9970, t550, t7021, t3946);
    (t94484, t94485, t94491, t94497, t94498, t94501, t94503, t94505, t94509, t94511, t94514)
}
