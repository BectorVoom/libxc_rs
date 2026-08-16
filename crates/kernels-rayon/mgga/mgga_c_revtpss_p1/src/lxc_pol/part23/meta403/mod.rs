//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta403(t17852: f64, t460: f64, t12050: f64, t3603: f64, t1284: f64, t5216: f64, t1204: f64, t5477: f64, t1269: f64, t3781: f64, t3766: f64, t1770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17853, t17854, t17861, t17864, t17879, t17880, t17887, t17888, t17934) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1772(t17852, t460, t12050, t3603, t1284, t5216, t1204, t5477, t1269, t3781, t3766, t1770);
    (t17853, t17854, t17861, t17864, t17879, t17880, t17887, t17888, t17934)
}
