//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1151/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1151(t31350: f64, t4921: f64, t30219: f64, t8473: f64, t4680: f64, t7426: f64, t8605: f64, t30468: f64, t4916: f64, t31346: f64, t4419: f64, t15386: f64, t31195: f64, t35749: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35792 = t31350 * t4921;
    let t35794 = t30219 * t8473;
    let t35795 = 0.47172138434406228102e-2_f64 * t35794;
    let t35797 = t7426 * t4680 * t8605;
    let t35798 = 0.42874018118069736972e-3_f64 * t35797;
    let t35799 = t30468 * t4916;
    let t35800 = 0.34299214494455789578e-2_f64 * t35799;
    let t35801 = t31346 * t4419;
    let t35804 = t31195 * t15386 * t35749;
    (t35792, t35795, t35798, t35800, t35801, t35804)
}
