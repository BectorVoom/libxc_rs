//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1608;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta319(t2516: f64, t5571: f64, t5566: f64, t72: f64, t757: f64, t1320: f64, t5567: f64, t5569: f64, t9395: f64, t2626: f64, t1856: f64, t2608: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13611, t13613, t13615, t13620, t13621, t13623, t13630, t13632) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1608(t2516, t5571, t5566, t72, t757, t1320, t5567, t5569, t9395, t2626, t1856, t2608);
    (t13611, t13613, t13615, t13620, t13621, t13623, t13630, t13632)
}
