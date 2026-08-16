//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2863/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863(t10696: f64, t14643: f64, t14648: f64, t14652: f64, t1553: f64, t18392: f64, t18435: f64, t18599: f64, t18612: f64, t227: f64, t23114: f64, t23148: f64, t23235: f64, t23238: f64, t23241: f64, t4343: f64, t4415: f64, t4416: f64, t5962: f64, t76421: f64, t775: f64, t830: f64, t832: f64, t853: f64) -> f64 {
    let t77118 = -360.0_f64 * t10696 * t23114 * t4415 * t775 - 12.0_f64 * t23148 * t4415 * t775 * t853 + 180.0_f64 * t14648 * t18435 * t4415 - 36.0_f64 * t14652 * t4415 * t5962 - 36.0_f64 * t18392 * t4415 * t4416 + 180.0_f64 * t18599 * t4343 * t4415 + 3.0_f64 * t227 * t76421 * t832 - 36.0_f64 * t14643 * t23238 + 9.0_f64 * t1553 * t18612 + 60.0_f64 * t23235 * t830 + 3.0_f64 * t23241 * t830;
    t77118
}
