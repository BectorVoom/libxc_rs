//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta748(t11466: f64, t300: f64, t51973: f64, t52035: f64, t52037: f64, t1633: f64, t3012: f64, t2986: f64, t4682: f64, t11465: f64, t1626: f64, t11509: f64, t4707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52238, t52337, t52346, t52397, t52406, t52407, t52430, t52440, t52443, t52459) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2537(t11466, t300, t51973, t52035, t52037, t1633, t3012, t2986, t4682, t11465, t1626, t11509, t4707);
    (t52238, t52337, t52346, t52397, t52406, t52407, t52430, t52440, t52443, t52459)
}
