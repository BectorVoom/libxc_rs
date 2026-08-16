//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1507/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1507(t14691: f64, t2747: f64, t837: f64, t2646: f64, t4450: f64, t10779: f64, t1548: f64, t10777: f64, t10811: f64, t4447: f64, t14676: f64, t2749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14693 = t2747 * t14691 * t837;
    let t14697 = t2747 * t4450 * t2646;
    let t14701 = t10779 * t1548 * t837;
    let t14703 = 0.10164000561857065645e-3_f64 * t10777 * t14701;
    let t14705 = 0.20007875121765877254e-2_f64 * t10811 * t4447;
    let t14707 = t2747 * t14676 * t2749;
    (t14693, t14697, t14701, t14703, t14705, t14707)
}
