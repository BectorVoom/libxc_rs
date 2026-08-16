//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk715;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk716;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta124(t3390: f64, t3391: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64, t1132: f64, t406: f64, t1139: f64, t281: f64, t2902: f64, t414: f64, t1146: f64, t698: f64, t1224: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3392, t3399) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk715(t3390, t3391, t3356, t3358, t3365, t3370, t3374);
        let (t3400, t3402, t3407, t3408, t3410, t3413, t3414, t3415, t3417) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk716(t1132, t3399, t3356, t406, t3391, t1139, t281, t2902, t414, t1146, t698, t1224, t240);
    (t3392, t3399, t3400, t3402, t3407, t3408, t3410, t3413, t3414, t3415, t3417)
}
