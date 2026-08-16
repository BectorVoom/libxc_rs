//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1880;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta544(t25900: f64, t96191: f64, t25904: f64, t26231: f64, t94802: f64, t2435: f64, t26355: f64, t2097: f64, t22: f64, t25937: f64, t94696: f64, t10115: f64, t2099: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t96192, t96193, t96195, t96197, t96204, t96206, t96210) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1880(t25900, t96191, t25904, t26231, t94802, t2435, t26355, t2097, t22, t25937, t94696, t10115, t2099);
    (t96192, t96193, t96195, t96197, t96204, t96206, t96210)
}
