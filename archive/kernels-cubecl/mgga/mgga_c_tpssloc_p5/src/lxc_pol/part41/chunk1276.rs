//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1276/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1276<F: Float>(t30297: F, t662: F, t29900: F, t8269: F, t1449: F, t666: F, t8184: F, t30063: F, t2: F, t29903: F, t30048: F, t30049: F, t30051: F, t30175: F, t30279: F, t30281: F, t30285: F, t30288: F, t30291: F, t30294: F, t8128: F, t8137: F) -> (F, F, F, F, F, F, F, F) {
    let t30298 = t30297 * t662;
    let t30301 = t29900 * t8269;
    let t30303 = t1449 * t666;
    let t30304 = t8184 * t30303;
    let t30307 = t1449 * t662;
    let t30308 = t30063 * t30307;
    let t30311 = t8184 * t2;
    let t30314 = -t30048 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t30049 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t30051 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t30279 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t29903 * t30281 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8128 * t30285 + t8128 * t30288 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t30291 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8128 * t30294 + F::cast_from(25.0_f64) / F::cast_from(72.0_f64) * t8137 * t30298 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t30301 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t8128 * t30304 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8137 * t30308 + F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t30175 * t30311;
    (t30298, t30301, t30303, t30304, t30307, t30308, t30311, t30314)
}
