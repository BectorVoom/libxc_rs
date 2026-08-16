//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1005;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta210(t4606: f64, t916: f64, t1600: f64, t2897: f64, t918: f64, t923: f64, t1606: f64, t698: f64, t2908: f64, t4574: f64, t141: f64, t4579: f64, t930: f64, t4583: f64, t2848: f64, t2892: f64, t2905: f64, t2906: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t4599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4607, t4614, t4615, t4617, t4620, t4622, t4623, t4625) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1005(t4606, t916, t1600, t2897, t918, t923, t1606, t698, t2908, t4574, t141, t4579, t930);
        let (t4626, t4628, t4629, t4631) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1006(t141, t4625, t4583, t930, t2848, t2892, t2905, t2906, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620, t4623);
    (t4607, t4614, t4615, t4617, t4620, t4622, t4623, t4625, t4626, t4628, t4629, t4631)
}
