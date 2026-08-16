//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2702/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2702<F: Float>(t1409: F, t1426: F, t67: F, t1434: F, t16558: F, t17635: F, t1864: F, t19322: F, t19323: F, t19331: F, t19334: F, t20218: F, t20219: F, t20222: F, t31: F, t3966: F, t3997: F, t5399: F, t628: F, t642: F, t65: F, t67060: F, t70458: F, t7445: F, t80: F) -> F {
    let t75361 = t1409 * t1426 * t67;
    let t75392 = -t19322 * t1864 * t16558 / F::cast_from(4.0_f64) - t75361 * t19323 / F::cast_from(2.0_f64) - t19322 * t7445 * t3966 / F::cast_from(2.0_f64) - t70458 * t65 * t80 / F::cast_from(12.0_f64) - t31 * t67060 * t65 * t80 / F::cast_from(12.0_f64) - t20218 * t628 * t80 / F::cast_from(12.0_f64) - t20219 * t642 / F::cast_from(12.0_f64) - t17635 * t1426 * t80 / F::cast_from(4.0_f64) - t19334 * t1426 * t80 / F::cast_from(4.0_f64) - t5399 * t3997 * t80 / F::cast_from(4.0_f64) - t20222 * t642 / F::cast_from(4.0_f64) - t19331 * t1434 / F::cast_from(4.0_f64);
    t75392
}
