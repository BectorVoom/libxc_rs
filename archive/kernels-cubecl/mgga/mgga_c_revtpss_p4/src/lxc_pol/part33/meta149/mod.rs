//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta149 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk775;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta149<F: Float>(t159: F, t550: F, t216: F, t1376: F, t2689: F, t1353: F, t1413: F, t547: F, t807: F, t2700: F, t535: F, t1369: F, t794: F, t1372: F, t2453: F, t546: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3943, t3944, t3950, t3951, t3952, t3953, t3956, t3957) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk775::<F>(t159, t550, t216, t1376, t2689, t1353, t1413, t547, t807, t2700, t535, t1369, t794);
        let (t3958, t3964) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk776::<F>(t1372, t3957, t2453, t546);
    (t3943, t3944, t3950, t3951, t3952, t3953, t3956, t3957, t3958, t3964)
}
