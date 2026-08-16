//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2012;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta589(t94483: f64, t64: f64, t9990: f64, t2482: f64, t596: f64, t7262: f64, t4021: f64, t25981: f64, t27: f64, t550: f64, t7021: f64, t25273: f64, t540: f64, t1372: f64, t2019: f64, t9951: f64, t2018: f64, t9646: f64, t9723: f64, t26014: f64, t2689: f64, t3994: f64, t7028: f64, t9845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94484, t94491, t94497, t94498, t94508, t94513, t94519) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2012(t94483, t64, t9990, t2482, t596, t7262, t4021, t25981, t27, t550, t7021, t25273, t540);
        let (t94520, t94523, t94526, t94527, t94537) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2013(t1372, t94519, t2019, t9951, t2018, t9646, t9723, t26014, t2689, t3994, t7028, t9845);
    (t94484, t94491, t94497, t94498, t94508, t94513, t94519, t94520, t94523, t94526, t94527, t94537)
}
