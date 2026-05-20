//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1755;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1756;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta479<F: Float>(t4345: F, t7045: F, t25234: F, t4349: F, t25227: F, t4353: F, t2661: F, t1565: F, t25222: F, t241: F, t25260: F, t820: F, t4368: F, t1955: F, t4469: F, t1579: F, t231: F, t836: F, t1559: F, t886: F, t7057: F, t1583: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27249, t27251, t27253, t27254, t27256, t27261) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1755::<F>(t4345, t7045, t25234, t4349, t25227, t4353, t2661, t1565, t25222, t241, t25260, t820);
        let (t27262, t27275, t27312, t27349, t27353) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1756::<F>(t27261, t4368, t1955, t4469, t1579, t231, t836, t1559, t886, t7057);
        let t27375 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1757::<F>(t1583, t775);
    (t27249, t27251, t27253, t27254, t27256, t27261, t27262, t27275, t27312, t27349, t27353, t27375)
}
