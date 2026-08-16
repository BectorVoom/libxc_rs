//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1126/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1126<F: Float>(t12463: F, t3080: F, t12458: F, t4232: F, t3931: F, t1569: F, t453: F, t1141: F, t2738: F, t4270: F, t9561: F, t3067: F) -> (F, F, F, F) {
    let t12465 = t3080 * t12463 / F::cast_from(2304.0_f64);
    let t12466 = t12458 * t4232;
    let t12467 = t3931 * t12466;
    let t12470 = t453 * t1569;
    let t12472 = t1141 * t12470 * t2738;
    let t12475 = t9561 * t4270;
    let t12477 = t3067 * t12475 / F::cast_from(3456.0_f64);
    (t12465, t12467, t12472, t12477)
}
