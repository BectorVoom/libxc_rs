//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2336/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2336<F: Float>(t1339: F, t22827: F, t54068: F, t550: F, t22779: F, t26319: F, t80837: F, t80843: F, t80848: F, t80857: F, t80859: F, t91261: F, t91263: F, t91268: F, t91272: F, t91276: F, t91279: F, t91282: F, t91284: F, t91287: F, t91290: F, t91294: F) -> F {
    let t91298 = t22827 * t1339 * t54068 * t550;
    let t91300 = t22779 * t26319;
    let t91301 = F::cast_from(0.56521858531796547196e-2_f64) * t91300;
    let t91302 = -t91261 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t91263 + F::cast_from(0.20186378047070195427e-3_f64) * t80837 - F::cast_from(0.14130464632949136799e-2_f64) * t80843 - t80848 - F::cast_from(0.40372756094140390854e-3_f64) * t91268 + F::cast_from(0.24223653656484234512e-2_f64) * t91272 + F::cast_from(0.12111826828242117256e-2_f64) * t91276 - t91279 / F::cast_from(768.0_f64) + t91282 + t91284 + t91287 - F::cast_from(0.40372756094140390854e-3_f64) * t80857 - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t80859 - F::cast_from(0.16956557559538964158e-1_f64) * t91290 + F::cast_from(0.24223653656484234512e-2_f64) * t91294 + F::cast_from(0.12111826828242117256e-2_f64) * t91298 - t91301;
    t91302
}
