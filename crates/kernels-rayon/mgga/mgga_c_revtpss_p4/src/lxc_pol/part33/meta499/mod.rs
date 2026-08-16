//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta499(t2122: f64, t25163: f64, t2139: f64, t3655: f64, t1256: f64, t7610: f64, t2138: f64, t3670: f64, t3666: f64, t3678: f64, t7613: f64, t3685: f64, t7607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t26795, t26821, t26822, t26824, t26827, t26832, t26836) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1806(t2122, t25163, t2139, t3655, t1256, t7610, t2138, t3670, t3666, t3678, t7613, t3685, t7607);
    (t26795, t26821, t26822, t26824, t26827, t26832, t26836)
}
