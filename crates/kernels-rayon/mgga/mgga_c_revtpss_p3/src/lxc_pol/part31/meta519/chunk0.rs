//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1878/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1878(t27384: f64, t27799: f64, t1113: f64, t1583: f64, t33: f64, t4537: f64, t1711: f64, t775: f64, t890: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25440: f64, t27158: f64, t27364: f64, t27368: f64, t27382: f64, t27407: f64, t27764: f64, t27770: f64, t27773: f64, t27777: f64, t27793: f64, t7087: f64, t7091: f64, t7200: f64, t7207: f64, t7783: f64, t7862: f64, t7869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27800 = t27799 * t27384;
    let t27802 = t1113 * t1583;
    let t27806 = t33 * t4537;
    let t27810 = t1711 * t775;
    let t27817 = t1711 * t890;
    let t27821 = 3.0_f64 * t27158 * t27764 + 3.0_f64 / 2.0_f64 * t2403 * t7087 * t7862 - 3.0_f64 / 2.0_f64 * t25206 * t27770 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t27773 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t27777 + 3.0_f64 / 2.0_f64 * t2403 * t7783 * t7200 + t1940 * t27364 * t33 / 2.0_f64 - t1940 * t27368 * t7207 / 2.0_f64 + t1940 * t7783 * t1113 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t25206 * t27793 - t1940 * t25440 * t7869 / 2.0_f64 + t27382 * t27800 - t1940 * t7091 * t27802 / 2.0_f64 - t1940 * t7091 * t27806 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t27810 + t1940 * t7087 * t1711 / 2.0_f64 - t1940 * t7091 * t27817 / 2.0_f64 - t27407;
    (t27800, t27802, t27806, t27810, t27817, t27821)
}
