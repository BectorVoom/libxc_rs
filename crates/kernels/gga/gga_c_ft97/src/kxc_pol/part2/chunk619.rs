//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 619/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk619<F: Float>(t299: F, t332: F, t4375: F, t113: F, t1273: F, t909: F, t1274: F, t505: F, t910: F, t992: F, t18: F, t1577: F, t1259: F, t1275: F, t2904: F, t4318: F, t4322: F, t5: F, t886: F, t889: F, t911: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t4376 = t4375 * t332;
    let t4377 = t4376 * t113;
    let t4380 = t1273 * t909;
    let t4381 = t332 * t113;
    let t4382 = t4380 * t4381;
    let t4385 = t1274 * t505;
    let t4391 = t910 * t992;
    let t4394 = t332 * t18;
    let t4395 = t4394 * t1577;
    let t4399 = piecewise3::<F>(t300, F::cast_from(0.0_f64), t5 * t4318 * t113 / F::cast_from(4.0_f64) + t4322 * t911 / F::cast_from(4.0_f64) + t5 * t1259 * t505 / F::cast_from(4.0_f64) + t2904 * t1275 / F::cast_from(4.0_f64) + t889 * t4377 / F::cast_from(4.0_f64) + t889 * t4382 / F::cast_from(4.0_f64) + t889 * t4385 / F::cast_from(4.0_f64) + t5 * t886 * t992 / F::cast_from(4.0_f64) + t889 * t4391 / F::cast_from(4.0_f64) - t889 * t4395 / F::cast_from(2.0_f64));
    (t4376, t4377, t4380, t4381, t4382, t4385, t4391, t4394, t4395, t4399)
}
