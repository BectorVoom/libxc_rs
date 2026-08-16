//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta121(t4398: f64, t762: f64, t162: f64, t2611: f64, t227: f64, t73: f64, t1544: f64, t853: f64, t1559: f64, t221: f64, t2485: f64, t2484: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4399, t4401, t4415, t4416, t4430, t4431) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk658(t4398, t762, t162, t2611, t227, t73, t1544, t853, t1559, t221, t2485, t2484);
    (t4399, t4401, t4415, t4416, t4430, t4431)
}
