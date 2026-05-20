//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2110/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2110<F: Float>(t25904: F, t98303: F, t786: F, t97961: F, t7286: F, t2439: F, t7925: F, t94391: F, t94383: F, t1444: F, t213: F, t225: F, t25921: F, t25924: F, t25930: F, t25931: F, t25961: F, t27837: F, t27846: F, t27858: F, t27902: F, t561: F, t7295: F, t94876: F, t98099: F, t98101: F, t98104: F, t98290: F, t98299: F) -> F {
    let t98305 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t98303;
    let t98308 = t786 * t97961;
    let t98310 = F::cast_from(0.14456046980341999104e-1_f64) * t98308 * t7286;
    let t98311 = t7925 * t2439;
    let t98312 = t94391 * t98311;
    let t98314 = t94383 * t98311;
    let t98318 = F::cast_from(0.8673628188205199462e0_f64) * t25921 * t27846 - F::cast_from(0.24093411633903331839e-3_f64) * t98099 - F::cast_from(0.17135234354032049604e-1_f64) * t98101 - F::cast_from(0.45699670022203476294e-2_f64) * t94876 - F::cast_from(0.96373646535613327357e-2_f64) * t98104 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t98290 * t225 * t561 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t27902 * t1444 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t25931 * t98299 - t98305 + F::cast_from(0.8673628188205199462e0_f64) * t27837 * t25961 + t98310 + F::cast_from(0.22849835011101738147e-2_f64) * t98312 - F::cast_from(0.17135234354032049604e-2_f64) * t98314 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t27858;
    t98318
}
