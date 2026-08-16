//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2217/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2217(t100135: f64, t11788: f64, t15787: f64, t15839: f64, t15895: f64, t15899: f64, t15922: f64, t16045: f64, t16098: f64, t16154: f64, t25580: f64, t27489: f64, t27493: f64, t27536: f64, t3177: f64, t3184: f64, t4839: f64, t4907: f64, t7131: f64, t93543: f64, t93548: f64, t93658: f64) -> f64 {
    let t100216 = 0.11433071498151929859e-2_f64 * t100135 * t16098 - 0.85748036236139473944e-3_f64 * t93543 * t4907 - 0.85748036236139473944e-3_f64 * t25580 * t15922 - 0.85748036236139473944e-3_f64 * t93658 * t15895 + 0.42874018118069736972e-3_f64 * t93548 * t15899 - 0.42874018118069736972e-3_f64 * t25580 * t16045 + 0.85748036236139473944e-3_f64 * t27493 * t15787 + 0.28582678745379824648e-3_f64 * t27489 * t3177 + 0.47637797908966374413e-3_f64 * t27489 * t3184 + 0.17149607247227894789e-2_f64 * t11788 * t7131 * t4839 + 0.17149607247227894789e-2_f64 * t27536 * t16154 + 0.85748036236139473944e-3_f64 * t27536 * t15839;
    t100216
}
