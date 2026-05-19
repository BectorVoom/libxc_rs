//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 717/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk717<F: Float>(t11306: F, t419: F, t11050: F, t1527: F, t11287: F, t11294: F, t11297: F, t11299: F, t11301: F, t11304: F, t8099: F, t8110: F, t8113: F, t8116: F, t8133: F) -> (F, F, F) {
    let t11307 = t419 * t11306;
    let t11309 = t1527 * t11050;
    let t11310 = t419 * t11309;
    let t11312 = -F::cast_from(0.45399899292181069959e-1_f64) * t11287 - F::cast_from(0.42562405586419753086e-2_f64) * t8099 - F::cast_from(0.28374937057613168724e-2_f64) * t8110 + F::cast_from(0.21281202793209876543e-2_f64) * t8113 + F::cast_from(0.28374937057613168724e-2_f64) * t8116 - F::cast_from(0.1134997482304526749e-1_f64) * t8133 + F::cast_from(0.62424861526748971195e-1_f64) * t11294 - t11297 - F::cast_from(0.14187468528806584362e-2_f64) * t11299 - F::cast_from(0.68099848938271604939e-1_f64) * t11301 - F::cast_from(0.2979368391049382716e-1_f64) * t11304 - F::cast_from(0.51074886703703703704e-1_f64) * t11307 + F::cast_from(0.38306165027777777778e-1_f64) * t11310;
    (t11307, t11310, t11312)
}
