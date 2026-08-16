//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1754;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1755;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1756;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta573(t141: f64, t3417: f64, t89837: f64, t1145: f64, t89849: f64, t89867: f64, t89871: f64, t89875: f64, t43764: f64, t89830: f64, t6449: f64, t3390: f64, t6442: f64, t43946: f64, t68255: f64, t81156: f64, t81158: f64, t89824: f64, t89828: f64, t89832: f64, t89839: f64, t89843: f64, t89847: f64, t89851: f64, t89855: f64, t43881: f64, t56236: f64, t68257: f64, t68399: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90402, t90405, t90408, t90411, t90414, t90417, t90419, t90420) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1754(t141, t3417, t89837, t1145, t89849, t89867, t89871, t89875, t43764, t89830, t6449, t3390);
        let (t90422, t90423, t90437) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1755(t6442, t43946, t68255, t81156, t81158, t89824, t89828, t89832, t89839, t89843, t89847, t89851, t89855);
        let t90449 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1756(t43881, t56236, t68257, t68399, t81230, t81232, t81234, t81236, t89865, t89869, t89873, t89877);
    (t90402, t90405, t90408, t90411, t90414, t90417, t90419, t90420, t90422, t90423, t90437, t90449)
}
