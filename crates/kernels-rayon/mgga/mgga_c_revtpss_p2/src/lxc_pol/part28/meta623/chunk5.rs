//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2212/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2212(t15822: f64, t25508: f64, t25525: f64, t4878: f64, t27450: f64, t3173: f64, t1047: f64, t15782: f64, t15791: f64, t15834: f64, t15952: f64, t16140: f64, t16149: f64, t16167: f64, t25522: f64, t27493: f64, t27536: f64, t3164: f64, t4825: f64, t4875: f64, t7132: f64, t93646: f64, t93764: f64) -> f64 {
    let t100063 = t15822 * t25508;
    let t100074 = t4878 * t25525;
    let t100078 = 0.57165357490759649296e-3_f64 * t27450 * t3173;
    let t100085 = 0.17149607247227894789e-2_f64 * t27493 * t15782 - 0.28582678745379824648e-3_f64 * t25522 * t16167 - 0.42874018118069736972e-3_f64 * t100063 * t3164 - 0.11433071498151929859e-2_f64 * t7132 * t15791 + 0.95275595817932748826e-3_f64 * t7132 * t15834 + 0.57165357490759649296e-3_f64 * t27536 * t16149 + 0.30488190661738479624e-2_f64 * t93646 * t4875 - 0.45732285992607719436e-2_f64 * t100074 * t1047 + t100078 - 0.57165357490759649296e-3_f64 * t93764 * t4825 - 0.57165357490759649296e-3_f64 * t25522 * t16140 - 0.57165357490759649296e-3_f64 * t25522 * t15952;
    t100085
}
