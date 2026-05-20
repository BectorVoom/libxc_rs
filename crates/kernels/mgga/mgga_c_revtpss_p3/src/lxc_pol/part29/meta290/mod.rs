//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta290<F: Float>(t3911: F, t3920: F, t3957: F, t3961: F, t3829: F, t4011: F, t547: F, t807: F, t2237: F, t240: F, t550: F, t816: F) -> (F, F, F, F, F, F) {
        let (t9695, t9697, t9703, t9705, t9707, t9709) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1177::<F>(t3911, t3920, t3957, t3961, t3829, t4011, t547, t807, t2237, t240, t550, t816);
    (t9695, t9697, t9703, t9705, t9707, t9709)
}
