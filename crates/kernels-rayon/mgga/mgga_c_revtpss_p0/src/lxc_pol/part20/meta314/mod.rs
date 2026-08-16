//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1222;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta314(t3363: f64, t5405: f64, t12787: f64, t12287: f64, t5308: f64, t12282: f64, t5312: f64, t1260: f64, t3650: f64, t3588: f64, t73: f64, t5352: f64, t3720: f64, t1209: f64, t3781: f64, t5330: f64, t3153: f64, t3601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12788, t12789, t12794, t12797, t12800, t12803, t12804) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1222(t3363, t5405, t12787, t12287, t5308, t12282, t5312, t1260, t3650, t3588, t73, t5352);
        let (t12805, t12808, t12809, t12810) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1223(t12804, t3720, t1209, t3781, t5330, t3153, t3601);
    (t12788, t12789, t12794, t12797, t12800, t12803, t12804, t12805, t12808, t12809, t12810)
}
