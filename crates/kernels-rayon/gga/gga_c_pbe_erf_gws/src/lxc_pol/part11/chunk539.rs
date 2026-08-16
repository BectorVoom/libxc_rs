//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 539/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk539(t127: f64, t1511: f64, t1519: f64, t1540: f64, t1555: f64, t1561: f64, t2879: f64, t2891: f64, t3648: f64, t3651: f64, t3654: f64, t3657: f64, t3661: f64, t3665: f64, t3668: f64, t496: f64) -> f64 {
    let t3671 = -t1511 + t3648 + t1519 + t3651 - t3654 + t1540 + t2879 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t496 * t3657 - t496 * t3661 / 2.0_f64 + t1555 + 0.146904e1_f64 * t2891 + t1561 + 0.587616e1_f64 * t127 * t3665 - 0.146904e1_f64 * t127 * t3668;
    t3671
}
