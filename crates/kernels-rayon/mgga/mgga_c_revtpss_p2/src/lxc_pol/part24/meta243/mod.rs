//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1005;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta243(t14485: f64, t2465: f64, t10073: f64, t4496: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t10867: f64, t225: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14486, t14512, t14523, t14524, t14525, t14533, t14545, t14546) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1005(t14485, t2465, t10073, t4496, t136, t1559, t2457, t10535, t10069, t10867, t225, t213);
    (t14486, t14512, t14523, t14524, t14525, t14533, t14545, t14546)
}
