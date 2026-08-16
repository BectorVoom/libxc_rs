//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2001;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2002;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta559<F: Float>(t228: F, t25273: F, t802: F, t25277: F, t2707: F, t25282: F, t9802: F, t243: F, t7021: F, t2732: F, t64: F, t9731: F, t2710: F, t826: F, t10631: F, t10886: F, t7028: F, t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92968, t92969, t92971, t92976, t92979, t92986) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2001::<F>(t228, t25273, t802, t25277, t2707, t25282, t9802, t243, t7021, t2732, t64, t9731);
        let (t92989, t92991, t92993, t92996, t92997) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2002::<F>(t2710, t826, t92986, t10631, t10886, t7028, t159, t8779, t218, t816, t10685, t1946);
    (t92968, t92969, t92971, t92976, t92979, t92986, t92989, t92991, t92993, t92996, t92997)
}
