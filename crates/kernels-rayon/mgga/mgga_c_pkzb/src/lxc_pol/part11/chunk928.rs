//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 928/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk928(t10092: f64, t3186: f64, t406: f64, t10083: f64, t3187: f64, t10038: f64, t10044: f64, t10047: f64, t10051: f64, t10056: f64, t10059: f64, t10061: f64, t10063: f64, t10067: f64, t10072: f64, t10077: f64, t10081: f64, t10085: f64, t10089: f64, t2380: f64, t3177: f64, t3185: f64, t3189: f64, t3196: f64, t3206: f64, t3209: f64, t385: f64, t8247: f64, t8319: f64, t8435: f64) -> (f64, f64, f64, f64, f64) {
    let t10093 = t3186 * t10092;
    let t10094 = t406 * t10093;
    let t10097 = t10083 * t3187;
    let t10098 = t406 * t10097;
    let t10101 = -t385 * t10038 / 96.0_f64 + 0.45732285992607719436e-2_f64 * t8319 * t3196 - 0.45732285992607719436e-2_f64 * t10044 * t3189 + 0.22866142996303859718e-2_f64 * t10047 * t3209 - 0.85748036236139473944e-3_f64 * t2380 * t10051 - 0.85748036236139473944e-3_f64 * t2380 * t10056 - 0.15244095330869239812e-2_f64 * t10059 + 0.48272968547752592738e-2_f64 * t10061 - t10063 * t3177 / 9.0_f64 - t8247 + 0.42874018118069736972e-3_f64 * t3206 * t10067 - 0.42874018118069736972e-3_f64 * t3206 * t10072 - 0.12862205435420921092e-2_f64 * t8435 * t10077 - 0.57165357490759649296e-3_f64 * t10081 - 0.21437009059034868486e-3_f64 * t3206 * t10085 - 0.85748036236139473944e-3_f64 * t3185 * t10089 + 0.85748036236139473944e-3_f64 * t3185 * t10094 + 0.42874018118069736972e-3_f64 * t3185 * t10098;
    (t10093, t10094, t10097, t10098, t10101)
}
