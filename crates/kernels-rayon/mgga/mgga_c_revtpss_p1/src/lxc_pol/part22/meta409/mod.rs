//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2008;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta409(t2777: f64, t5759: f64, t2439: f64, t1398: f64, t1892: f64, t4086: f64, t543: f64, t2782: f64, t5659: f64, t72: f64, t686: f64, t4101: f64, t136: f64, t1883: f64, t2457: f64, t10139: f64, t13926: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14202, t14203, t14207, t14209, t14215, t14216, t14218) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2008(t2777, t5759, t2439, t1398, t1892, t4086, t543, t2782, t5659, t72, t686, t4101);
        let (t14219, t14220, t14221, t14224) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2009(t136, t1883, t2457, t10139, t13926, t543);
    (t14202, t14203, t14207, t14209, t14215, t14216, t14218, t14219, t14220, t14221, t14224)
}
