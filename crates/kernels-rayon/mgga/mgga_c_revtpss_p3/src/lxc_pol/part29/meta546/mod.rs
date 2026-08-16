//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta546(t25878: f64, t96239: f64, t26230: f64, t9670: f64, t25895: f64, t94633: f64, t25899: f64, t94639: f64, t1358: f64, t2439: f64, t7506: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96240, t96242, t96243, t96245, t96246, t96248, t96249, t96253) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1883(t25878, t96239, t26230, t9670, t25895, t94633, t25899, t94639, t1358, t2439, t7506, t785);
    (t96240, t96242, t96243, t96245, t96246, t96248, t96249, t96253)
}
