//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1879;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta543(t2453: f64, t2458: f64, t7399: f64, t2070: f64, t41154: f64, t11064: f64, t7427: f64, t25876: f64, t26304: f64, t25894: f64, t94398: f64, t122: f64, t72: f64, t7506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t95948, t95964, t95976, t96186, t96187, t96188, t96191) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1879(t2453, t2458, t7399, t2070, t41154, t11064, t7427, t25876, t26304, t25894, t94398, t122, t72, t7506);
    (t95948, t95964, t95976, t96186, t96187, t96188, t96191)
}
