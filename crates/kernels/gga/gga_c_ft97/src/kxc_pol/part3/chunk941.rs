//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 941/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk941<F: Float>(t18532: F, t2607: F, t2606: F, t1882: F, t5087: F, t5083: F, t5079: F, t5075: F, t17720: F, t17724: F, t17729: F, t17734: F, t17738: F, t17742: F, t17746: F, t17751: F, t17755: F, t17759: F, t17763: F) -> (F, F, F, F, F, F) {
    let t18533 = t2607 * t18532;
    let t18534 = t2606 * t18533;
    let t18538 = t1882 * t5087;
    let t18540 = t1882 * t5083;
    let t18542 = t1882 * t5079;
    let t18544 = t1882 * t5075;
    let t18557 = -F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t17720 + t17724 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17729 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t17734 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t17738 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17742 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17746 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t17751 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t17755 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17759 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t17763;
    (t18534, t18538, t18540, t18542, t18544, t18557)
}
