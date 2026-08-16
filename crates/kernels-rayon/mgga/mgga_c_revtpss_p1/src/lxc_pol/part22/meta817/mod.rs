//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta817 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2927;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta817(t1904: f64, t2439: f64, t9640: f64, t5718: f64, t9292: f64, t14274: f64, t2435: f64, t4078: f64, t5599: f64, t689: f64, t13734: f64, t1445: f64, t10175: f64, t14090: f64, t14100: f64, t9671: f64, t1357: f64, t14269: f64, t1358: f64, t14066: f64, t212: f64, t13746: f64, t686: f64, t72: f64, t9680: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47800, t47802, t47805, t47808, t47811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2927(t1904, t2439, t9640, t5718, t9292, t14274, t2435, t4078, t5599, t689, t13734, t1445);
        let (t47813, t47816, t47819, t47825, t47832) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2928(t10175, t14090, t14100, t9671, t1357, t14269, t689, t1358, t14066, t212, t13746, t686, t72, t9680);
    (t47800, t47802, t47805, t47808, t47811, t47813, t47816, t47819, t47825, t47832)
}
