//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2621;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta663(t21013: f64, t3767: f64, t3782: f64, t3628: f64, t4186: f64, t5351: f64, t3626: f64, t12910: f64, t17283: f64, t17375: f64, t17448: f64, t17605: f64, t1791: f64, t21001: f64, t21004: f64, t21008: f64, t3625: f64, t5320: f64, t5323: f64, t5335: f64, t5343: f64, t5402: f64, t5407: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t21014, t21017) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2621(t21013, t3767, t3782);
        let (t21020, t21021, t21022, t21027) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2622(t3628, t4186, t5351, t3626, t12910, t17283, t17375, t17448, t17605, t1791, t21001, t21004, t21008, t21014, t21017, t3625, t5320, t5323, t5335, t5343, t5402, t5407);
    (t21014, t21017, t21020, t21021, t21022, t21027)
}
