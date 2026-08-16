//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta206 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta206(t460: f64, t5462: f64, t3302: f64, t3603: f64, t1248: f64, t5332: f64, t1269: f64, t1287: f64, t1794: f64, t487: f64, t5284: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5463, t5464, t5465, t5466, t5470, t5474, t5477) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk898(t460, t5462, t3302, t3603, t1248, t5332, t1269, t1287, t1794, t487, t5284, t3781);
    (t5463, t5464, t5465, t5466, t5470, t5474, t5477)
}
