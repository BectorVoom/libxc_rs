//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1617/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1617(t16179: f64, t16182: f64, t1045: f64, t373: f64, t1042: f64, t1041: f64, t11656: f64, t12021: f64, t16140: f64, t16144: f64, t16149: f64, t16154: f64, t16160: f64, t16165: f64, t16167: f64, t16172: f64, t1671: f64, t3124: f64, t3127: f64, t4837: f64, t4869: f64, t4875: f64) -> (f64, f64) {
    let t16183 = t16179 + t16182;
    let t16185 = t373 * t16183 * t1045;
    let t16186 = t1042 * t16185;
    let t16189 = -0.28582678745379824648e-3_f64 * t3127 * t16140 + 0.28582678745379824648e-3_f64 * t3127 * t16144 + 0.28582678745379824648e-3_f64 * t4837 * t16149 + 0.85748036236139473944e-3_f64 * t4837 * t16154 + t16160 + 0.15244095330869239812e-2_f64 * t11656 * t4875 + t16165 - 0.14291339372689912324e-3_f64 * t3127 * t16167 - 0.23818898954483187207e-3_f64 * t3127 * t16172 + 0.21437009059034868486e-3_f64 * t12021 * t1671 + 0.42874018118069736972e-3_f64 * t3124 * t4869 + 0.21437009059034868486e-3_f64 * t1041 * t16186;
    (t16183, t16189)
}
