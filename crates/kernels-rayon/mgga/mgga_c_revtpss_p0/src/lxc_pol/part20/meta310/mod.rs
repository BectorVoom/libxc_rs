//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta310(t1294: f64, t3790: f64, t3737: f64, t1284: f64, t3552: f64, t1204: f64, t3766: f64, t3153: f64, t3588: f64) -> (f64, f64, f64, f64, f64) {
        let (t12695, t12696, t12699, t12702, t12705) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1212(t1294, t3790, t3737, t1284, t3552, t1204, t3766, t3153, t3588);
    (t12695, t12696, t12699, t12702, t12705)
}
