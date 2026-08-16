//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta112 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk660;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta112<F: Float>(t2662: F, t2664: F, t2661: F, t240: F, t596: F, t243: F, t816: F, t813: F, t2482: F, t27: F, t849: F, t136: F, t854: F, t221: F, t775: F, t26: F, t66: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2665, t2666, t2668, t2670, t2672, t2674) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk660::<F>(t2662, t2664, t2661, t240, t596, t243, t816, t813, t2482, t27, t849);
        let (t2675, t2677, t2678, t2681) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk661::<F>(t136, t854, t221, t775, t2674, t26, t66);
    (t2665, t2666, t2668, t2670, t2672, t2674, t2675, t2677, t2678, t2681)
}
