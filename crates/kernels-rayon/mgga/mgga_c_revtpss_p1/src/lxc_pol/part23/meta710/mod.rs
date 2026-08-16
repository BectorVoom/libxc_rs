//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta710 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2466;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta710(t5603: f64, t9692: f64, t136: f64, t2457: f64, t5774: f64, t9674: f64, t10073: f64, t13731: f64, t3915: f64, t5721: f64, t9288: f64, t2439: f64, t3895: f64, t5775: f64, t14293: f64, t9664: f64, t14103: f64, t9285: f64, t13726: f64, t9303: f64, t13725: f64, t1445: f64, t14082: f64, t3920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47863, t47886, t47899, t47904, t47907) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2466(t5603, t9692, t136, t2457, t5774, t9674, t10073, t13731, t3915, t5721, t9288, t2439, t3895, t5775);
        let (t47920, t47932, t47938, t47942, t47944) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2467(t14293, t9664, t14103, t9285, t9674, t13726, t9303, t13725, t1445, t2439, t14082, t3920);
    (t47863, t47886, t47899, t47904, t47907, t47920, t47932, t47938, t47942, t47944)
}
