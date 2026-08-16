//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2362/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2362(t100828: f64, t100833: f64, t100835: f64, t100838: f64, t100840: f64, t100854: f64, t100861: f64, t100863: f64, t1266: f64, t29493: f64, t4026: f64, t5107: f64, t7983: f64, t8103: f64, t97930: f64, t97932: f64, t97935: f64, t97937: f64, t97941: f64, t97942: f64, t97947: f64, t97949: f64) -> f64 {
    let t105099 = -2.0_f64 * t1266 * t29493 - 2.0_f64 * t4026 * t8103 - 2.0_f64 * t5107 * t7983 + t100828 - t100833 - t100835 + t100838 - t100840 + t100854 + t100861 - t100863 + t97930 - t97932 - t97935 - t97937 + t97941 + t97942 - t97947 - t97949;
    t105099
}
