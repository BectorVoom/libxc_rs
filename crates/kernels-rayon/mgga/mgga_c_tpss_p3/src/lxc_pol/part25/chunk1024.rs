//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1024/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1024(t14108: f64, t150: f64, t190: f64, t4579: f64, t725: f64, t681: f64, t125: f64, t4758: f64, t2175: f64, t2177: f64, t4715: f64, t8325: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14164 = t150 * t14108;
    let t14165 = t14164 * t190;
    let t14166 = t725 * t4579;
    let t14167 = t681 * t14166;
    let t14168 = 4.0_f64 * t14167;
    let t14169 = t125 * t4758;
    let t14171 = t2175 * t14169 * t2177;
    let t14174 = t125 * t4715;
    let t14176 = t2175 * t14174 * t8325;
    (t14165, t14168, t14169, t14171, t14174, t14176)
}
