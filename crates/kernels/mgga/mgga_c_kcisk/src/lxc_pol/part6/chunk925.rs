//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 925/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk925<F: Float>(t5886: F, t8078: F, t1411: F, t2152: F, t8072: F, t1450: F, t3785: F, t2231: F, t7831: F, t1415: F, t30494: F, t3776: F, t1340: F, t14208: F, t30489: F, t2232: F, t25351: F) -> (F, F, F, F, F, F, F, F) {
    let t31152 = t5886 * t8078;
    let t31153 = t1411 * t31152;
    let t31165 = t8072 * t2152;
    let t31166 = t1450 * t31165;
    let t31167 = t3785 * t31166;
    let t31168 = t1411 * t31167;
    let t31170 = t7831 * t2231;
    let t31171 = t1450 * t31170;
    let t31172 = t1415 * t31171;
    let t31173 = t1411 * t31172;
    let t31175 = t3776 * t30494;
    let t31176 = t1340 * t31175;
    let t31177 = t1411 * t31176;
    let t31179 = t14208 * t30489;
    let t31180 = t1340 * t31179;
    let t31181 = t1411 * t31180;
    let t31183 = t25351 * t2232;
    (t31153, t31165, t31168, t31170, t31173, t31177, t31181, t31183)
}
