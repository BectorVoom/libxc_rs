//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1828;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1829;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta524<F: Float>(t802: F, t92968: F, t25282: F, t9802: F, t243: F, t7021: F, t64: F, t9731: F, t2710: F, t826: F, t10631: F, t10886: F, t7028: F, t159: F, t8779: F, t218: F, t816: F, t10685: F, t1946: F, t10671: F, t7033: F, t25255: F, t2689: F, t10690: F, t1945: F, t9646: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92969, t92975, t92978, t92986, t92988, t92991) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1828::<F>(t802, t92968, t25282, t9802, t243, t7021, t64, t9731, t2710, t826, t10631, t10886, t7028);
        let (t92993, t92995, t92997, t92999, t93001, t93007) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1829::<F>(t159, t8779, t218, t816, t10685, t1946, t10671, t7033, t25255, t2689, t10690, t1945, t9646);
    (t92969, t92975, t92978, t92986, t92988, t92991, t92993, t92995, t92997, t92999, t93001, t93007)
}
