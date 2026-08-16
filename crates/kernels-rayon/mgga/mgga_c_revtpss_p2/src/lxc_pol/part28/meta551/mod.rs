//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta551 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta551(t843: f64, t2247: f64, t25138: f64, t38: f64, t1925: f64, t2251: f64, t45963: f64, t6957: f64, t10309: f64, t25105: f64, t45972: f64, t45958: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t92612, t92644, t92666, t92684, t92687, t92690, t92699) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2001(t843, t2247, t25138, t38, t1925, t2251, t45963, t6957, t10309, t25105, t45972, t45958);
    (t92612, t92644, t92666, t92684, t92687, t92690, t92699)
}
