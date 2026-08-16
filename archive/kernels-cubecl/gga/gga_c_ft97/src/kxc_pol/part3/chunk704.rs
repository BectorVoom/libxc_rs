//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 704/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk704<F: Float>(t1557: F, t586: F, t1037: F, t1771: F, t3524: F, t458: F, t2: F, t9224: F, t1775: F, t3503: F, t3507: F, t3500: F) -> (F, F, F, F, F, F, F) {
    let t12796 = t586 * t1557;
    let t12809 = t1771 * t1037;
    let t12816 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t458 * t3524;
    let t12823 = t9224 * t2;
    let t12834 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1775 * t3503;
    let t12836 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1775 * t3507;
    let t12839 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1775 * t3500;
    (t12796, t12809, t12816, t12823, t12834, t12836, t12839)
}
