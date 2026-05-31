//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 356/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk356<F: Float>(t1341: F, t2231: F, t1415: F, t1411: F, t1450: F, t2152: F, t1340: F, t1220: F, t1335: F, t2110: F, t2174: F, t2179: F, t2215: F, t412: F) -> (F, F, F, F, F, F, F) {
    let t2232 = t1341 * t2231;
    let t2233 = t1415 * t2232;
    let t2234 = t1411 * t2233;
    let t2236 = t1450 * t2152;
    let t2237 = t1340 * t2236;
    let t2238 = t1411 * t2237;
    let t2240 = t2110 * t412 - F::cast_from(0.193e0_f64) * t1220 * t2174 + t1335 + F::cast_from(0.16581944444444444444e-2_f64) * t2179 + F::cast_from(0.24872916666666666666e-2_f64) * t2215 - F::cast_from(0.24872916666666666666e-2_f64) * t2234 + F::cast_from(0.16581944444444444444e-2_f64) * t2238;
    (t2232, t2233, t2234, t2236, t2237, t2238, t2240)
}
