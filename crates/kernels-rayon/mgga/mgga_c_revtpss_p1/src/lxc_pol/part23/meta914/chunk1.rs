//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2948/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2948(t4595: f64, t63677: f64, t4636: f64, t64336: f64, t15101: f64, t19327: f64, t15421: f64, t19331: f64, t19324: f64, t52508: f64, t19250: f64, t19256: f64, t52224: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78303 = 6.0_f64 * t63677 * t4595;
    let t78305 = 0.48245938496077605201e2_f64 * t64336 * t4636;
    let t78307 = 6.0_f64 * t15101 * t19327;
    let t78309 = 0.48245938496077605201e2_f64 * t15421 * t19331;
    let t78311 = 0.2894756309764656312e3_f64 * t52508 * t19324;
    let t78313 = 0.96491876992155210402e2_f64 * t15421 * t19250;
    let t78315 = 0.1551780387578202009e4_f64 * t52224 * t19256;
    (t78303, t78305, t78307, t78309, t78311, t78313, t78315)
}
