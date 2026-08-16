//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta208 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1231;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1232;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1233;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta208(t108: f64, t5911: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t5896: f64, t5899: f64, t5902: f64, t5908: f64, t97: f64, t114: f64, t655: f64, t2335: f64, t4261: f64, t5892: f64, t69: f64, t508: f64, t4303: f64, t4306: f64, t2498: f64, t2518: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t2628: f64, t2632: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5912, t5915) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1231(t108, t5911, t105, t109, t1507, t1510, t5896, t5899, t5902, t5908, t97);
        let (t5916, t5920) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1232(t114, t5915, t655, t2335, t4261, t5892, t69);
        let t5921 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1233(t508, t5920);
        let (t5924, t5925, t5926) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1234(t4303, t4306, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2628, t2632);
    (t5912, t5915, t5916, t5920, t5921, t5924, t5925, t5926)
}
