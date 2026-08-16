//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta298(t6622: f64, t73: f64, t17934: f64, t5330: f64, t5327: f64, t5362: f64, t1803: f64, t5326: f64, t5323: f64, t12772: f64, t6639: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t21040, t21049, t21053, t21063, t21088, t21090, t21091) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1083(t6622, t73, t17934, t5330, t5327, t5362, t1803, t5326, t5323, t12772, t6639, t3625);
    (t21040, t21049, t21053, t21063, t21088, t21090, t21091)
}
