//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta549 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta549(t7289: f64, t96282: f64, t26277: f64, t94776: f64, t25950: f64, t26292: f64, t26230: f64, t94764: f64, t94768: f64, t94763: f64, t26234: f64, t94890: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t96284, t96287, t96289, t96292, t96294, t96296) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1886(t7289, t96282, t26277, t94776, t25950, t26292, t26230, t94764, t94768, t94763, t26234, t94890);
    (t96284, t96287, t96289, t96292, t96294, t96296)
}
