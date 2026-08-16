//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta538(t25386: f64, t95536: f64, t26518: f64, t9285: f64, t25299: f64, t2061: f64, t22: f64, t25402: f64, t93140: f64, t25310: f64, t26506: f64, t2439: f64, t7398: f64, t780: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t95537, t95540, t95542, t95546, t95548, t95551, t95562) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1849(t25386, t95536, t26518, t9285, t25299, t2061, t22, t25402, t93140, t25310, t26506, t2439, t7398, t780, t785);
    (t95537, t95540, t95542, t95546, t95548, t95551, t95562)
}
