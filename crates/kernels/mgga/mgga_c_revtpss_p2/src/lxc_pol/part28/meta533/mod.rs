//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1975;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta533<F: Float>(t1497: F, t640: F, t77: F, t4241: F, t84: F, t1470: F, t2242: F, t1923: F, t1928: F, t25106: F, t28078: F, t28081: F, t28086: F, t28090: F, t28093: F, t6954: F, t6958: F, t6974: F, t6978: F, t7702: F, t7706: F, t7716: F, t7720: F) -> (F, F, F, F, F, F) {
        let (t28104, t28105, t28108, t28109, t28112, t28115) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1975::<F>(t1497, t640, t77, t4241, t84, t1470, t2242, t1923, t1928, t25106, t28078, t28081, t28086, t28090, t28093, t6954, t6958, t6974, t6978, t7702, t7706, t7716, t7720);
    (t28104, t28105, t28108, t28109, t28112, t28115)
}
