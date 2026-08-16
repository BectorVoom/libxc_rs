//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta873 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3035;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta873(t11044: f64, t14983: f64, t14485: f64, t15014: f64, t9303: f64, t10510: f64, t14987: f64, t14991: f64, t41066: f64, t10982: f64, t1568: f64, t9646: f64, t252: f64, t2769: f64, t2782: f64, t4533: f64, t886: f64, t10995: f64, t11049: f64, t14990: f64, t14986: f64, t2453: f64, t10506: f64, t2458: f64, t4470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51231, t51233, t51237, t51239, t51241, t51246) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3035(t11044, t14983, t14485, t15014, t9303, t10510, t14987, t14991, t41066, t10982, t1568, t9646);
        let (t51251, t51256, t51258, t51259, t51262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3036(t252, t2769, t2782, t4533, t886, t10995, t11049, t14990, t14986, t2453, t10506, t2458, t4470);
    (t51231, t51233, t51237, t51239, t51241, t51246, t51251, t51256, t51258, t51259, t51262)
}
