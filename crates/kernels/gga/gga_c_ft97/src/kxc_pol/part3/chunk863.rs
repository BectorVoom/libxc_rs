//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 863/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk863<F: Float>(t17409: F, t609: F, t144: F, t2185: F, t4668: F, t616: F, t4724: F, t9276: F, t1882: F, t4811: F, t13152: F, t13187: F, t13190: F, t17377: F, t17381: F, t17385: F, t17390: F, t17394: F, t17398: F, t17402: F, t17406: F, t1901: F, t446: F) -> (F, F, F) {
    let t17410 = t17409 * t609;
    let t17411 = t144 * t17410;
    let t17415 = t2185 * t616 * t4668;
    let t17418 = t9276 * t4724;
    let t17419 = t144 * t17418;
    let t17422 = t1882 * t4811;
    let t17425 = -F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t17377 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t17381 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t17385 - t13152 + t446 * t17390 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t17394 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t17398 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t17402 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t17406 - t446 * t17411 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t17415 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t17419 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17422 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13187 + t13190;
    (t17410, t17418, t17425)
}
