//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta403(t473: f64, t5412: f64, t13147: f64, t487: f64, t460: f64, t12050: f64, t13045: f64, t13141: f64, t3603: f64, t1284: f64, t5216: f64, t1770: f64, t3766: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17821, t17846, t17847, t17853, t17854, t17861, t17934) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1450(t473, t5412, t13147, t487, t460, t12050, t13045, t13141, t3603, t1284, t5216, t1770, t3766);
    (t17821, t17846, t17847, t17853, t17854, t17861, t17934)
}
