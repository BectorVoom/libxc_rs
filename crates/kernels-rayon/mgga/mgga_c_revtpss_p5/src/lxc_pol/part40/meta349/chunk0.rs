//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1189/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1189(t14113: f64, t2482: f64, t4104: f64, t10073: f64, t5737: f64, t1419: f64, t1882: f64, t4086: f64, t543: f64, t2782: f64, t555: f64, t5658: f64) -> (f64, f64, f64, f64, f64) {
    let t14114 = t2482 * t14113;
    let t14116 = 0.19514881078765566038e-1_f64 * t14114 * t4104;
    let t14120 = t10073 * t5737;
    let t14122 = t1419 * t1882;
    let t14124 = t4086 * t14122 * t543;
    let t14126 = 0.10975748638225852664e-1_f64 * t2782 * t14124;
    let t14127 = t555 * t5658;
    (t14116, t14120, t14122, t14126, t14127)
}
