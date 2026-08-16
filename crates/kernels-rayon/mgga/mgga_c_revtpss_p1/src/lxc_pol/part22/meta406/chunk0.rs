//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2001/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2001(t14114: f64, t4104: f64, t10073: f64, t5737: f64, t1419: f64, t1882: f64, t4086: f64, t543: f64, t2782: f64, t555: f64, t5658: f64, t4114: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14116 = 0.19514881078765566038e-1_f64 * t14114 * t4104;
    let t14120 = t10073 * t5737;
    let t14122 = t1419 * t1882;
    let t14124 = t4086 * t14122 * t543;
    let t14126 = 0.10975748638225852664e-1_f64 * t2782 * t14124;
    let t14127 = t555 * t5658;
    let t14129 = t4086 * t14127 * t543;
    let t14131 = 0.10975748638225852664e-1_f64 * t2782 * t14129;
    let t14140 = t4114 * t1882;
    (t14116, t14120, t14122, t14124, t14126, t14127, t14129, t14131, t14140)
}
