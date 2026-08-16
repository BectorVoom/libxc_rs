//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta154 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta154(t2782: f64, t4089: f64, t1419: f64, t545: f64, t869: f64, t689: f64, t136: f64, t555: f64, t2457: f64, t3964: f64, t4086: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4090, t4092, t4093, t4094, t4096, t4099, t4100, t4101) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk838(t2782, t4089, t1419, t545, t869, t689, t136, t555, t2457, t3964, t4086, t786);
    (t4090, t4092, t4093, t4094, t4096, t4099, t4100, t4101)
}
