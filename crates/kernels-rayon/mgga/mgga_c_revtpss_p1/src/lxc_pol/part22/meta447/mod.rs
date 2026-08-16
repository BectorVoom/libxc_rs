//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta447(t2: f64, t895: f64, t580: f64, t265: f64, t22: f64, t4567: f64, t1610: f64, t2875: f64, t2924: f64, t1596: f64, t2873: f64, t2876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15093, t15094, t15096, t15098, t15100, t15101, t15103) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2094(t2, t895, t580, t265, t22, t4567, t1610, t2875, t2924, t1596, t2873, t2876);
    (t15093, t15094, t15096, t15098, t15100, t15101, t15103)
}
