//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2212/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2212<F: Float>(t15822: F, t25508: F, t25525: F, t4878: F, t27450: F, t3173: F, t1047: F, t15782: F, t15791: F, t15834: F, t15952: F, t16140: F, t16149: F, t16167: F, t25522: F, t27493: F, t27536: F, t3164: F, t4825: F, t4875: F, t7132: F, t93646: F, t93764: F) -> F {
    let t100063 = t15822 * t25508;
    let t100074 = t4878 * t25525;
    let t100078 = F::cast_from(0.57165357490759649296e-3_f64) * t27450 * t3173;
    let t100085 = F::cast_from(0.17149607247227894789e-2_f64) * t27493 * t15782 - F::cast_from(0.28582678745379824648e-3_f64) * t25522 * t16167 - F::cast_from(0.42874018118069736972e-3_f64) * t100063 * t3164 - F::cast_from(0.11433071498151929859e-2_f64) * t7132 * t15791 + F::cast_from(0.95275595817932748826e-3_f64) * t7132 * t15834 + F::cast_from(0.57165357490759649296e-3_f64) * t27536 * t16149 + F::cast_from(0.30488190661738479624e-2_f64) * t93646 * t4875 - F::cast_from(0.45732285992607719436e-2_f64) * t100074 * t1047 + t100078 - F::cast_from(0.57165357490759649296e-3_f64) * t93764 * t4825 - F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t16140 - F::cast_from(0.57165357490759649296e-3_f64) * t25522 * t15952;
    t100085
}
