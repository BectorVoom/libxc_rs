//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta288(t1082: f64, t11173: f64, t3298: f64, t989: f64, t3059: f64, t3291: f64, t4980: f64, t994: f64, t3151: f64, t999: f64, t3304: f64, t4995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12111, t12116, t12119, t12122, t12123, t12124, t12127) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1155(t1082, t11173, t3298, t989, t3059, t3291, t4980, t994, t3151, t999, t3304, t4995);
    (t12111, t12116, t12119, t12122, t12123, t12124, t12127)
}
