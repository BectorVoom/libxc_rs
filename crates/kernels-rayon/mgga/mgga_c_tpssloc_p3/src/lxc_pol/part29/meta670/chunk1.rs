//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2241/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2241(t1339: f64, t22827: f64, t54068: f64, t550: f64, t22779: f64, t26319: f64, t80837: f64, t80843: f64, t80848: f64, t80857: f64, t80859: f64, t91261: f64, t91263: f64, t91268: f64, t91272: f64, t91276: f64, t91279: f64, t91282: f64, t91284: f64, t91287: f64, t91290: f64, t91294: f64) -> f64 {
    let t91298 = t22827 * t1339 * t54068 * t550;
    let t91300 = t22779 * t26319;
    let t91301 = 0.56521858531796547196e-2_f64 * t91300;
    let t91302 = -t91261 / 96.0_f64 - 5.0_f64 / 192.0_f64 * t91263 + 0.20186378047070195427e-3_f64 * t80837 - 0.14130464632949136799e-2_f64 * t80843 - t80848 - 0.40372756094140390854e-3_f64 * t91268 + 0.24223653656484234512e-2_f64 * t91272 + 0.12111826828242117256e-2_f64 * t91276 - t91279 / 768.0_f64 + t91282 + t91284 + t91287 - 0.40372756094140390854e-3_f64 * t80857 - 35.0_f64 / 576.0_f64 * t80859 - 0.16956557559538964158e-1_f64 * t91290 + 0.24223653656484234512e-2_f64 * t91294 + 0.12111826828242117256e-2_f64 * t91298 - t91301;
    t91302
}
