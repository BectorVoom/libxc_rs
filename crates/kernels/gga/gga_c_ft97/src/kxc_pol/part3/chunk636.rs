//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 636/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk636<F: Float>(t299: F, t332: F, t5473: F, t113: F, t1273: F, t1274: F, t992: F, t1259: F, t1275: F, t333: F, t4322: F, t4635: F, t5: F, t5430: F, t889: F) -> (F, F, F, F, F, F, F) {
    let t300 = F::cast_from(10000000.0_f64) <= t299;
    let t5474 = t5473 * t332;
    let t5475 = t5474 * t113;
    let t5478 = t1273 * t1273;
    let t5479 = t5478 * t332;
    let t5480 = t5479 * t113;
    let t5483 = t1274 * t992;
    let t5490 = piecewise3::<F>(t300, F::cast_from(0.0_f64), t5 * t5430 * t113 / F::cast_from(4.0_f64) + t4322 * t1275 / F::cast_from(2.0_f64) + t5 * t1259 * t992 / F::cast_from(2.0_f64) + t889 * t5475 / F::cast_from(4.0_f64) + t889 * t5480 / F::cast_from(4.0_f64) + t889 * t5483 / F::cast_from(2.0_f64) + t5 * t333 * t4635 / F::cast_from(4.0_f64));
    (t5474, t5475, t5478, t5479, t5480, t5483, t5490)
}
