//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1979/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1979<F: Float>(t108379: F, t7515: F, t102361: F, t102363: F, t102364: F, t102367: F, t108282: F, t1444: F, t2097: F, t22386: F, t22395: F, t25921: F, t25924: F, t25930: F, t27837: F, t27868: F, t28850: F, t28911: F, t28918: F, t30105: F, t30227: F, t30296: F, t30308: F, t7292: F, t7295: F, t7296: F, t7511: F, t75188: F, t7523: F, t96392: F, t97933: F) -> F {
    let t109609 = t108379 * t7515;
    let t109628 = -F::cast_from(0.17347256376410398924e1_f64) * t27868 * t28911 * t75188 - F::cast_from(0.17347256376410398924e1_f64) * t97933 * t28918 + t102361 + t102363 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t96392 * t30105 + F::cast_from(0.8673628188205199462e0_f64) * t108282 * t7523 + F::cast_from(0.72280234901709995518e-2_f64) * t109609 + F::cast_from(0.26341796731742046394e1_f64) * t7511 * t22395 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t30308 * t1444 - F::cast_from(0.45699670022203476294e-2_f64) * t102364 + t102367 - F::cast_from(0.4336814094102599731e0_f64) * t7292 * t30296 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t2097 * t22386 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t28850 - F::cast_from(0.8673628188205199462e0_f64) * t25921 * t30227;
    t109628
}
