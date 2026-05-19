//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 833/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk833<F: Float>(t4729: F, t981: F, t1633: F, t3011: F, t3014: F, t972: F, t2848: F, t3037: F, t4571: F, t4576: F, t4581: F, t4585: F) -> (F, F, F, F, F, F) {
    let t4731 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t4729;
    let t4732 = t3011 * t1633;
    let t4733 = t3014 * t972;
    let t4734 = t4732 * t4733;
    let t4736 = F::cast_from(0.17315859105681463759e2_f64) * t981 * t4734;
    let t4742 = t3037 + F::cast_from(0.27777777777777777778e-2_f64) * t2848 + F::cast_from(0.27777777777777777778e-2_f64) * t4571 - F::cast_from(0.55555555555555555555e-2_f64) * t4576 + F::cast_from(0.16666666666666666667e-1_f64) * t4581 - F::cast_from(0.83333333333333333333e-2_f64) * t4585;
    (t4731, t4732, t4733, t4734, t4736, t4742)
}
