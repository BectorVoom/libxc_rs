//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1192;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta304(t3519: f64, t444: f64, t439: f64, t1187: f64, t3497: f64, t3523: f64, t1175: f64, t3495: f64, t1188: f64, t1189: f64, t3515: f64, t1170: f64, t3471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12485 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1192(t3519, t444);
        let (t12486, t12487, t12488, t12491, t12494, t12497, t12500, t12501, t12504) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1193(t12485, t439, t1187, t3497, t3523, t1175, t3495, t1188, t1189, t3515, t1170, t3471);
    (t12485, t12486, t12487, t12488, t12491, t12494, t12497, t12500, t12501, t12504)
}
