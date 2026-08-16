//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1144/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1144(t14127: f64, t4086: f64, t543: f64, t2782: f64, t1882: f64, t4114: f64, t2482: f64, t122: f64, t4003: f64, t72: f64, t1398: f64, t676: f64) -> (f64, f64, f64, f64) {
    let t14129 = t4086 * t14127 * t543;
    let t14131 = 0.10975748638225852664e-1_f64 * t2782 * t14129;
    let t14140 = t4114 * t1882;
    let t14141 = t2482 * t14140;
    let t14143 = t4003 * t72 * t122;
    let t14144 = t676 * t1398;
    (t14131, t14141, t14143, t14144)
}
