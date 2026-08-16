//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk848;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta209(t4631: f64, t935: f64, t915: f64, t1609: f64, t2926: f64, t934: f64, t2924: f64, t2848: f64, t2930: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t1614: f64, t945: f64, t1622: f64, t953: f64, t2906: f64, t2950: f64, t2957: f64, t4599: f64, t4607: f64, t4615: f64, t4617: f64, t4620: f64, t4623: f64, t4626: f64, t4629: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4632, t4634, t4635, t4636, t4638, t4644) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk848(t4631, t935, t915, t1609, t2926, t934, t2924, t2848, t2930, t4571, t4576, t4581, t4585);
        let (t4647, t4652, t4669) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk849(t1614, t945, t1622, t953, t2848, t2906, t2950, t2957, t4571, t4576, t4581, t4585, t4599, t4607, t4615, t4617, t4620, t4623, t4626, t4629);
    (t4632, t4634, t4635, t4636, t4638, t4644, t4647, t4652, t4669)
}
