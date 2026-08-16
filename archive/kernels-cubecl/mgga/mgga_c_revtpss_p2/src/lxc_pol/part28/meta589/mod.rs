//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2058;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta589<F: Float>(t1416: F, t94545: F, t25978: F, t3970: F, t240: F, t25981: F, t2661: F, t9935: F, t25987: F, t9775: F, t25986: F, t9769: F, t4014: F, t25972: F, t9923: F, t2453: F, t4086: F, t64: F, t9795: F, t2018: F, t40688: F, t46808: F, t7256: F, t9784: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94546, t94548, t94550, t94552, t94554, t94557) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2058::<F>(t1416, t94545, t25978, t3970, t240, t25981, t2661, t9935, t25987, t9775, t25986, t9769);
        let (t94559, t94561, t94564, t94565, t94569, t94570) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2059::<F>(t25978, t4014, t25972, t9923, t2453, t4086, t64, t9795, t2018, t40688, t46808, t7256, t9784);
    (t94546, t94548, t94550, t94552, t94554, t94557, t94559, t94561, t94564, t94565, t94569, t94570)
}
