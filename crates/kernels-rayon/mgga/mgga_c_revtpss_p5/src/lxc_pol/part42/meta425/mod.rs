//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta425(t1501: f64, t4292: f64, t21881: f64, t93: f64, t10208: f64, t625: f64, t46157: f64, t69: f64, t2289: f64, t2339: f64, t655: f64, t10199: f64, t2195: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t109153, t109242, t116912, t116919, t116926, t116929, t117183) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1487(t1501, t4292, t21881, t93, t10208, t625, t46157, t69, t2289, t2339, t655, t10199, t2195);
    (t109153, t109242, t116912, t116919, t116926, t116929, t117183)
}
