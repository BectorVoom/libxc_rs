//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1540;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta294(t2438: f64, t886: f64, t138: f64, t10504: f64, t2434: f64, t123: f64, t2465: f64, t213: f64, t2760: f64, t215: f64, t231: f64, t268: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10505, t10506, t10507, t10509, t10510, t10511, t10513, t10518) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1540(t2438, t886, t138, t10504, t2434, t123, t2465, t213, t2760, t215, t231, t268, t836);
    (t10505, t10506, t10507, t10509, t10510, t10511, t10513, t10518)
}
