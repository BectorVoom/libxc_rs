//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta226(t3264: f64, t5989: f64, t1661: f64, t3270: f64, t3274: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t1100: f64, t3287: f64, t1107: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t5991, t5992, t5993, t5999, t6000, t6006, t6008) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1044(t3264, t5989, t1661, t3270, t3274, t4721, t5973, t5977, t5981, t1100, t3287, t1107);
    (t5991, t5992, t5993, t5999, t6000, t6006, t6008)
}
