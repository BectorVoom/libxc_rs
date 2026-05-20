//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2009;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta593<F: Float>(t94473: F, t1389: F, t3964: F, t92986: F, t7028: F, t9736: F, t9737: F, t26009: F, t9802: F, t64: F, t9990: F, t2482: F, t596: F, t7262: F, t4021: F, t25981: F, t27: F, t550: F, t7021: F, t25273: F, t540: F, t1372: F, t2019: F, t9951: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94474, t94477, t94479, t94484, t94491, t94497) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2009::<F>(t94473, t1389, t3964, t92986, t7028, t9736, t9737, t26009, t9802, t64, t9990, t2482, t596, t7262);
        let (t94498, t94508, t94513, t94519, t94520, t94522) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2010::<F>(t4021, t94497, t2482, t25981, t27, t550, t7021, t25273, t540, t1372, t2019, t9951);
    (t94474, t94477, t94479, t94484, t94491, t94497, t94498, t94508, t94513, t94519, t94520, t94522)
}
