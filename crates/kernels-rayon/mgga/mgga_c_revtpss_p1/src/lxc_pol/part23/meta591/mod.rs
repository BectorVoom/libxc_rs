//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2228;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta591(t23740: f64, t23753: f64, t954: f64, t1621: f64, t19275: f64, t1634: f64, t6205: f64, t1633: f64, t19303: f64, t1610: f64, t6141: f64, t2874: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23754, t23755, t23758, t23761, t23764, t23767, t23769) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2228(t23740, t23753, t954, t1621, t19275, t1634, t6205, t1633, t19303, t1610, t6141, t2874);
    (t23754, t23755, t23758, t23761, t23764, t23767, t23769)
}
