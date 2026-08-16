//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1484;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta425(t31032: f64, t31284: f64, t116912: f64, t31261: f64, t10208: f64, t69: f64, t96: f64, t100: f64, t1513: f64, t2339: f64, t31027: f64, t31268: f64, t10199: f64, t116: f64, t31292: f64, t1913: f64, t8302: f64, t2192: f64, t5789: f64, t2184: f64, t5808: f64, t31328: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t117484, t117497, t117499, t117500, t117505, t117510) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1484(t31032, t31284, t116912, t31261, t10208, t69, t96, t100, t1513, t2339, t31027, t31268);
        let (t117544, t117758, t117772, t117774, t117781, t117783) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1485(t10199, t2339, t116, t31292, t1913, t8302, t2192, t5789, t2184, t5808, t31328, t575);
    (t117484, t117497, t117499, t117500, t117505, t117510, t117544, t117758, t117772, t117774, t117781, t117783)
}
