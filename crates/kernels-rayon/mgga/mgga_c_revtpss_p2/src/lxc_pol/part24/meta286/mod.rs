//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta286 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta286(t1040: f64, t19696: f64, t16509: f64, t4891: f64, t16584: f64, t19463: f64, t366: f64, t11710: f64, t6267: f64, t3091: f64, t3172: f64, t6311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19697, t19738, t19741, t19773, t19785, t19786, t19826) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1066(t1040, t19696, t16509, t4891, t16584, t19463, t366, t11710, t6267, t3091, t3172, t6311);
    (t19697, t19738, t19741, t19773, t19785, t19786, t19826)
}
