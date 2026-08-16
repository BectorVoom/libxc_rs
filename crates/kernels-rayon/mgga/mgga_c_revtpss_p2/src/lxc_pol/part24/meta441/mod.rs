//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1397;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta441(t3869: f64, t39538: f64, t39427: f64, t39535: f64, t3853: f64, t3857: f64, t73: f64, t9940: f64, t820: f64, t843: f64, t9991: f64, t1386: f64, t2237: f64, t2482: f64, t235: f64, t46475: f64, t239: f64, t4000: f64, t596: f64, t72: f64, t245: f64, t136: f64, t4010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47138, t47140, t47142, t47152, t47171, t47194, t47198) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1397(t3869, t39538, t39427, t39535, t3853, t3857, t73, t9940, t820, t843, t9991, t1386, t2237, t2482);
        let (t47203, t47215, t47248, t47273) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1398(t235, t46475, t239, t820, t2482, t4000, t596, t72, t9940, t245, t136, t4010);
    (t47138, t47140, t47142, t47152, t47171, t47194, t47198, t47203, t47215, t47248, t47273)
}
