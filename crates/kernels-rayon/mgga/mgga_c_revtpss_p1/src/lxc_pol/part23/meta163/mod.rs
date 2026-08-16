//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta163(t2908: f64, t4574: f64, t141: f64, t4579: f64, t930: f64, t4583: f64, t2848: f64, t2892: f64, t2905: f64, t2906: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t4599: f64, t4607: f64, t4615: f64, t4617: f64, t4620: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t4622, t4623, t4625, t4626, t4628, t4629, t4631) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk988(t2908, t4574, t141, t4579, t930, t4583, t2848, t2892, t2905, t2906, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620);
    (t4622, t4623, t4625, t4626, t4628, t4629, t4631)
}
