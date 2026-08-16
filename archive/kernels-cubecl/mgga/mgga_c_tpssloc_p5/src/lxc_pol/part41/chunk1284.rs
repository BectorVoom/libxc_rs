//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1284/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1284<F: Float>(t30534: F, t574: F, t1849: F, t8273: F, t1774: F, t2199: F, t6287: F, t6468: F, t510: F, t1268: F, t19451: F, t2200: F, t2202: F, t28002: F, t28007: F, t28030: F, t4028: F, t652: F, t7458: F, t7676: F, t8260: F, t8274: F, t8278: F, t8280: F) -> (F, F, F, F, F, F, F) {
    let t30535 = t30534 * t574;
    let t30538 = t8273 * t1849;
    let t30543 = t1774 * t8273;
    let t30558 = t6287 * t2199;
    let t30565 = t2199 * t6468;
    let t30574 = t510 * t30534;
    let t30581 = F::cast_from(2.0_f64) * t1268 * t30535 + F::cast_from(4.0_f64) * t1268 * t30538 + F::cast_from(2.0_f64) * t1268 * t30565 - F::cast_from(2.0_f64) * t19451 * t2200 + F::cast_from(2.0_f64) * t19451 * t2202 - F::cast_from(4.0_f64) * t2200 * t28002 - F::cast_from(2.0_f64) * t2200 * t28030 + F::cast_from(4.0_f64) * t2202 * t28002 + F::cast_from(2.0_f64) * t2202 * t28007 - F::cast_from(4.0_f64) * t30543 * t652 - F::cast_from(2.0_f64) * t30558 * t652 - F::cast_from(2.0_f64) * t30574 * t652 - F::cast_from(4.0_f64) * t4028 * t8260 - F::cast_from(4.0_f64) * t4028 * t8274 + F::cast_from(4.0_f64) * t4028 * t8278 + F::cast_from(4.0_f64) * t4028 * t8280 - F::cast_from(4.0_f64) * t7458 * t8260 - F::cast_from(4.0_f64) * t7458 * t8274 + F::cast_from(4.0_f64) * t7676 * t8278 + F::cast_from(4.0_f64) * t7676 * t8280;
    (t30535, t30538, t30543, t30558, t30565, t30574, t30581)
}
