//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2188/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2188(t1113: f64, t4343: f64, t1583: f64, t3351: f64, t27799: f64, t63164: f64, t100975: f64, t100978: f64, t100982: f64, t100988: f64, t100993: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t25752: f64, t25760: f64, t25784: f64, t27368: f64, t27382: f64, t27770: f64, t27793: f64, t27806: f64, t4541: f64, t7091: f64, t7783: f64, t7869: f64, t92775: f64, t92819: f64, t98637: f64) -> f64 {
    let t100997 = t1113 * t4343;
    let t101012 = t3351 * t1583;
    let t101016 = t27799 * t63164;
    let t101021 = 2.0_f64 * t27382 * t100975 + 6.0_f64 * t25206 * t100978 - 3.0_f64 * t27382 * t100982 - 3.0_f64 * t92819 * t27793 - 3.0_f64 * t25206 * t100988 - t1940 * t25440 * t27806 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t100993 + 3.0_f64 * t2403 * t1963 * t100997 - t1940 * t27368 * t25784 / 2.0_f64 - 3.0_f64 * t92819 * t27770 + 3.0_f64 * t4541 * t7783 * t25752 - t1940 * t92775 * t7869 / 2.0_f64 - t1940 * t7091 * t101012 / 2.0_f64 + 2.0_f64 * t27382 * t101016 - 3.0_f64 * t98637 * t25760;
    t101021
}
