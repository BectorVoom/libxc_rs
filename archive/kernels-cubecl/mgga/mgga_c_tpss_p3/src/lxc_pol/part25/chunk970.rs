//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 970/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk970<F: Float>(t13335: F, t36: F, t70: F, t4580: F, t602: F, t1306: F, t3426: F, t3432: F, t1290: F, t3462: F, t1314: F, t13321: F, t13322: F, t13325: F, t13331: F, t3427: F, t4574: F, t4581: F, t4584: F, t616: F, t85: F) -> (F, F) {
    let t13336 = t36 * t13335;
    let t13337 = t13336 * t70;
    let t13340 = t4580 * t602;
    let t13345 = t3426 * t1306;
    let t13348 = t3432 * t1306;
    let t13351 = t1290 * t3462;
    let t13358 = -t13321 * t13322 / F::cast_from(6.0_f64) - t13325 * t85 / F::cast_from(12.0_f64) - t4574 * t616 / F::cast_from(12.0_f64) - t13331 * t85 / F::cast_from(12.0_f64) - t13337 * t85 / F::cast_from(12.0_f64) - t13340 * t85 / F::cast_from(12.0_f64) - t4581 * t616 / F::cast_from(12.0_f64) - t13345 * t85 / F::cast_from(6.0_f64) - t13348 * t85 / F::cast_from(6.0_f64) - t13351 * t85 / F::cast_from(6.0_f64) - t4584 * t616 / F::cast_from(6.0_f64) - t3427 * t1314 / F::cast_from(6.0_f64);
    (t13336, t13358)
}
