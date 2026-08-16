//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2372;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2373;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta618(t10115: f64, t225: f64, t880: f64, t10866: f64, t232: f64, t235: f64, t2723: f64, t10666: f64, t221: f64, t2484: f64, t2485: f64, t2482: f64, t2719: f64, t596: f64, t10852: f64, t10858: f64, t10863: f64, t10868: f64, t820: f64, t843: f64, t10874: f64, t27: f64, t10872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40317, t40318, t40321, t40322, t40325, t40333, t40336) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2372(t10115, t225, t880, t10866, t232, t235, t2723, t10666, t221, t2484, t2485, t2482, t2719, t596);
        let (t40337, t40345, t40349, t40355) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2373(t10852, t40336, t10858, t10863, t10868, t820, t843, t10874, t2482, t27, t10872, t221, t2485);
    (t40317, t40318, t40321, t40322, t40325, t40333, t40337, t40345, t40349, t40355)
}
