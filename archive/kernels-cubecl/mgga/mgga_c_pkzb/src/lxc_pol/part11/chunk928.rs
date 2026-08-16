//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 928/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk928<F: Float>(t10092: F, t3186: F, t406: F, t10083: F, t3187: F, t10038: F, t10044: F, t10047: F, t10051: F, t10056: F, t10059: F, t10061: F, t10063: F, t10067: F, t10072: F, t10077: F, t10081: F, t10085: F, t10089: F, t2380: F, t3177: F, t3185: F, t3189: F, t3196: F, t3206: F, t3209: F, t385: F, t8247: F, t8319: F, t8435: F) -> (F, F, F, F, F) {
    let t10093 = t3186 * t10092;
    let t10094 = t406 * t10093;
    let t10097 = t10083 * t3187;
    let t10098 = t406 * t10097;
    let t10101 = -t385 * t10038 / F::cast_from(96.0_f64) + F::cast_from(0.45732285992607719436e-2_f64) * t8319 * t3196 - F::cast_from(0.45732285992607719436e-2_f64) * t10044 * t3189 + F::cast_from(0.22866142996303859718e-2_f64) * t10047 * t3209 - F::cast_from(0.85748036236139473944e-3_f64) * t2380 * t10051 - F::cast_from(0.85748036236139473944e-3_f64) * t2380 * t10056 - F::cast_from(0.15244095330869239812e-2_f64) * t10059 + F::cast_from(0.48272968547752592738e-2_f64) * t10061 - t10063 * t3177 / F::cast_from(9.0_f64) - t8247 + F::cast_from(0.42874018118069736972e-3_f64) * t3206 * t10067 - F::cast_from(0.42874018118069736972e-3_f64) * t3206 * t10072 - F::cast_from(0.12862205435420921092e-2_f64) * t8435 * t10077 - F::cast_from(0.57165357490759649296e-3_f64) * t10081 - F::cast_from(0.21437009059034868486e-3_f64) * t3206 * t10085 - F::cast_from(0.85748036236139473944e-3_f64) * t3185 * t10089 + F::cast_from(0.85748036236139473944e-3_f64) * t3185 * t10094 + F::cast_from(0.42874018118069736972e-3_f64) * t3185 * t10098;
    (t10093, t10094, t10097, t10098, t10101)
}
