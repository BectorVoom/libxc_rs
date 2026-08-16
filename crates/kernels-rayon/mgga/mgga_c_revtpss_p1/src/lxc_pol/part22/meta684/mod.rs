//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta684 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta684(t5: f64, t21812: f64, t117: f64, t5892: f64, t625: f64, t10208: f64, t5891: f64, t665: f64, t4263: f64, t4287: f64, t5916: f64, t2339: f64, t5915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21813, t21814, t21818, t21820, t21821, t21824, t21827, t21829) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2672(t5, t21812, t117, t5892, t625, t10208, t5891, t665, t4263, t4287, t5916, t2339, t5915);
    (t21813, t21814, t21818, t21820, t21821, t21824, t21827, t21829)
}
