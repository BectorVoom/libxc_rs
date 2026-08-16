//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta683(t43813: f64, t1209: f64, t13126: f64, t17708: f64, t1203: f64, t12626: f64, t225: f64, t480: f64, t12627: f64, t1269: f64, t44842: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t45232, t45371, t45384, t45385, t45386, t45427, t45438) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2424(t43813, t1209, t13126, t17708, t1203, t12626, t225, t480, t12627, t1269, t44842, t487);
    (t45232, t45371, t45384, t45385, t45386, t45427, t45438)
}
