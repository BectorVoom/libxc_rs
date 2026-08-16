//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1173/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1173<F: Float>(t1882: F, t35118: F, t12664: F, t33090: F, t23997: F, t26526: F, t1017: F, t12703: F, t140378: F, t140382: F, t140383: F, t144: F, t148403: F, t148408: F, t148412: F, t1901: F, t2142: F, t2179: F, t2185: F, t27256: F, t32962: F, t33060: F, t33125: F, t33227: F, t3408: F, t3429: F, t35050: F, t35192: F, t3590: F, t40792: F, t40945: F, t446: F, t47659: F, t51151: F, t574: F, t5975: F, t605: F, t6615: F, t7312: F, t7400: F, t7407: F, t9144: F, t925: F, t95842: F) -> (F, F, F) {
    let t149132 = t1882 * t35118;
    let t149141 = t12664 * t33090;
    let t149191 = t23997 * t26526;
    let t149196 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t140378 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t149132 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t5975 * t6615 + t140382 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t47659 * t95842 * t27256 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t149141 + t446 * t574 * t605 * t7407 * t3408 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t2179 * t7400 * t3408 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t3590 * t7312 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t40945 * t35192 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t9144 * t33060 * t925 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t9144 * t33125 * t925 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t51151 * t148403 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t40792 * t32962 * t3429 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t12703 * t148408 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t12703 * t148412 - t446 * t574 * t33227 * t1017 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2185 * t2142 * t35050 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t149191 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t140383;
    (t149141, t149191, t149196)
}
