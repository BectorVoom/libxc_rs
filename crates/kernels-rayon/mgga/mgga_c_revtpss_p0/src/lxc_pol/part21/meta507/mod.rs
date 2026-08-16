//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2126;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2127;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2128;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta507(t15984: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64, t1011: f64, t11672: f64, t11675: f64, t11881: f64, t11886: f64, t12004: f64, t15952: f64, t15959: f64, t15965: f64, t15970: f64, t15975: f64, t1675: f64, t3127: f64, t4783: f64, t4892: f64, t4899: f64, t3252: f64, t4574: f64, t15145: f64, t4915: f64, t15149: f64, t15154: f64, t4919: f64, t15130: f64, t15135: f64, t1012: f64, t11821: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15986, t15987) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2126(t15984, t3091, t1014, t140);
        let (t15988, t15991) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2127(t15987, t4579, t1011, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let t15993 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2128(t140, t3252);
        let (t15994, t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2129(t15993, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
    (t15987, t15988, t15991, t15993, t15994, t15996, t15997, t16000, t16003, t16006, t16009, t16012)
}
