//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 351/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk351<F: Float>(t1339: F, t2178: F, t2059: F, t425: F, t1359: F, t2077: F, t2084: F, t1355: F, t2083: F, t306: F) -> (F, F, F, F) {
    let t2179 = t1339 * t2178;
    let t2181 = t425 * t2059;
    let t2188 = F::cast_from(0.1982e-1_f64) * t2084 - t1359 - F::cast_from(0.41275e-2_f64) * t2077;
    let t2191 = t1355 * t2083 / F::cast_from(4.0_f64) + t306 * t2188 / F::cast_from(2.0_f64);
    (t2179, t2181, t2188, t2191)
}
