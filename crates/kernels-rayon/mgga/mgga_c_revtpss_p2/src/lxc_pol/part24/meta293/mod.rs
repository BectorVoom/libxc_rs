//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta293(t12230: f64, t6438: f64, t3523: f64, t6534: f64, t12555: f64, t6518: f64, t3801: f64, t6748: f64, t1209: f64, t6695: f64, t460: f64, t487: f64, t6564: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20651, t20671, t20678, t20692, t20697, t20700, t20753) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1076(t12230, t6438, t3523, t6534, t12555, t6518, t3801, t6748, t1209, t6695, t460, t487, t6564);
    (t20651, t20671, t20678, t20692, t20697, t20700, t20753)
}
