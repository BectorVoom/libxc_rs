//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1678;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1679;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta276(t521: f64, t9413: f64, t182: f64, t2490: f64, t2495: f64, t9368: f64, t1340: f64, t2626: f64, t4038: f64, t2491: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t9415, t9417) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1678(t521, t9413, t182, t2490);
        let t9419 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1679(t2495, t9368, t9417);
        let (t9421, t9422, t9425) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1680(t1340, t9419, t2626, t4038, t2491, t745, t9368);
    (t9415, t9417, t9419, t9421, t9422, t9425)
}
