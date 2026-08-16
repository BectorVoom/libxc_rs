//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1390/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1390<F: Float>(t3242: F, t39103: F, t136: F, t3297: F, t43713: F, t43717: F, t43721: F, t43725: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43748: F, t43750: F) -> (F, F, F) {
    let t43752 = t3242 * t39103;
    let t43754 = t136 * t3297 * t43752;
    let t43756 = -F::cast_from(0.99342e0_f64) * t43713 - F::cast_from(0.11038e0_f64) * t43717 + F::cast_from(0.298026e1_f64) * t43721 + F::cast_from(0.66228e0_f64) * t43725 + F::cast_from(0.80513333333333333333e0_f64) * t43727 - F::cast_from(0.24154e1_f64) * t43729 + F::cast_from(0.20128333333333333334e1_f64) * t43734 - F::cast_from(0.72462e1_f64) * t43737 - F::cast_from(0.80513333333333333332e0_f64) * t43740 + F::cast_from(0.108693e2_f64) * t43743 + F::cast_from(0.24154e1_f64) * t43746 - F::cast_from(0.53675555555555555556e0_f64) * t43748 - F::cast_from(0.44729629629629629629e0_f64) * t43750 - F::cast_from(0.82785e-1_f64) * t43754;
    (t43752, t43754, t43756)
}
