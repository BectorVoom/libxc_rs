//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2505;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta687(t221: f64, t461: f64, t462: f64, t624: f64, t1250: f64, t606: f64, t1235: f64, t3661: f64, t371: f64, t676: f64, t1236: f64, t2434: f64, t1208: f64, t12689: f64, t225: f64, t480: f64, t3671: f64, t3672: f64, t12625: f64, t458: f64, t456: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44797, t44799, t44823, t44829) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2505(t221, t461, t462, t624, t1250, t606, t1235, t3661, t371, t676, t1236, t2434);
        let (t44831, t44832, t44833, t44838, t44842, t44843, t44865) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2506(t1208, t12689, t225, t480, t3671, t3672, t371, t676, t12625, t458, t456, t43813);
    (t44797, t44799, t44823, t44829, t44831, t44832, t44833, t44838, t44842, t44843, t44865)
}
