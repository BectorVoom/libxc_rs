//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1041/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1041(t10956: f64, t1679: f64, t467: f64, t9099: f64, t33857: f64, t33861: f64, t33867: f64, t33869: f64, t33874: f64, t33894: f64, t33960: f64, t33984: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36617 = 2.0_f64 * t1679 * t10956 * t467;
    let t36619 = 4.0_f64 * t1679 * t9099;
    let t36823 = 0.12579236915841660827e-2_f64 * t33857;
    let t36825 = 35.0_f64 / 216.0_f64 * t33861;
    let t36828 = 0.85748036236139473944e-3_f64 * t33867;
    let t36829 = 0.15724046144802076034e-2_f64 * t33869;
    let t36833 = 0.10718504529517434243e-2_f64 * t33874;
    let t36838 = 0.28582678745379824648e-3_f64 * t33894;
    let t36876 = 0.7640625e-2_f64 * t33960;
    let t36889 = 0.37737710747524982482e-2_f64 * t33984;
    (t36617, t36619, t36823, t36825, t36828, t36829, t36833, t36838, t36876, t36889)
}
