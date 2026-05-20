//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2217/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2217<F: Float>(t100135: F, t11788: F, t15787: F, t15839: F, t15895: F, t15899: F, t15922: F, t16045: F, t16098: F, t16154: F, t25580: F, t27489: F, t27493: F, t27536: F, t3177: F, t3184: F, t4839: F, t4907: F, t7131: F, t93543: F, t93548: F, t93658: F) -> F {
    let t100216 = F::cast_from(0.11433071498151929859e-2_f64) * t100135 * t16098 - F::cast_from(0.85748036236139473944e-3_f64) * t93543 * t4907 - F::cast_from(0.85748036236139473944e-3_f64) * t25580 * t15922 - F::cast_from(0.85748036236139473944e-3_f64) * t93658 * t15895 + F::cast_from(0.42874018118069736972e-3_f64) * t93548 * t15899 - F::cast_from(0.42874018118069736972e-3_f64) * t25580 * t16045 + F::cast_from(0.85748036236139473944e-3_f64) * t27493 * t15787 + F::cast_from(0.28582678745379824648e-3_f64) * t27489 * t3177 + F::cast_from(0.47637797908966374413e-3_f64) * t27489 * t3184 + F::cast_from(0.17149607247227894789e-2_f64) * t11788 * t7131 * t4839 + F::cast_from(0.17149607247227894789e-2_f64) * t27536 * t16154 + F::cast_from(0.85748036236139473944e-3_f64) * t27536 * t15839;
    t100216
}
