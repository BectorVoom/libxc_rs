//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta653(t5219: f64, t7627: f64, t105202: f64, t1209: f64, t29109: f64, t29135: f64, t3566: f64, t8190: f64, t460: f64, t5251: f64, t8945: f64, t26921: f64, t8205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t105433, t105442, t105499, t105509, t105512, t105519, t105530, t105558) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2104(t5219, t7627, t105202, t1209, t29109, t29135, t3566, t8190, t460, t5251, t8945, t26921, t8205);
    (t105433, t105442, t105499, t105509, t105512, t105519, t105530, t105558)
}
