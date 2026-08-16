//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2058;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta589(t1416: f64, t94545: f64, t25978: f64, t3970: f64, t240: f64, t25981: f64, t2661: f64, t9935: f64, t25987: f64, t9775: f64, t25986: f64, t9769: f64, t4014: f64, t25972: f64, t9923: f64, t2453: f64, t4086: f64, t64: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94546, t94548, t94550, t94552, t94554, t94557) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2058(t1416, t94545, t25978, t3970, t240, t25981, t2661, t9935, t25987, t9775, t25986, t9769);
        let (t94559, t94561, t94564, t94565, t94569, t94570) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2059(t25978, t4014, t25972, t9923, t2453, t4086, t64, t9795, t2018, t40688, t46808, t7256, t9784);
    (t94546, t94548, t94550, t94552, t94554, t94557, t94559, t94561, t94564, t94565, t94569, t94570)
}
