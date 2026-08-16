//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2948;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2949;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta829(t1868: f64, t9940: f64, t5577: f64, t588: f64, t5585: f64, t4010: f64, t5591: f64, t13921: f64, t221: f64, t4018: f64, t4019: f64, t2661: f64, t3924: f64, t3992: f64, t5651: f64, t5608: f64, t1882: f64, t9956: f64, t13774: f64, t5675: f64, t9934: f64, t4056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48347, t48394, t48417, t48432, t48445, t48449) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2948(t1868, t9940, t5577, t588, t5585, t4010, t5591, t13921, t221, t4018, t4019, t2661, t3924, t3992, t5651);
        let (t48453, t48458, t48462, t48466) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2949(t2661, t3924, t3992, t5608, t1882, t4010, t9956, t13774, t5675, t9934, t1868, t4056);
    (t48347, t48394, t48417, t48432, t48445, t48449, t48453, t48458, t48462, t48466)
}
