//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1479;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta422<F: Float>(t1518: F, t2178: F, t2681: F, t64: F, t10207: F, t111: F, t116: F, t21813: F, t5876: F, t670: F, t5891: F, t665: F, t1513: F, t4287: F, t5915: F, t5920: F, t648: F, t21881: F, t94: F, t4245: F, t1501: F, t4292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t35739, t46089, t46157, t75439, t85360, t105872) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1479::<F>(t1518, t2178, t2681, t64, t10207, t111, t116, t21813, t5876, t670, t5891, t665);
        let (t105875, t105880, t108710, t108714, t109150, t109153) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1480::<F>(t1513, t4287, t5915, t665, t5920, t648, t21881, t94, t1518, t4245, t1501, t4292);
    (t35739, t46089, t46157, t75439, t85360, t105872, t105875, t105880, t108710, t108714, t109150, t109153)
}
