//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 669/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk669<F: Float>(t9723: F, t9727: F, t9730: F, t9520: F, t9768: F, t9765: F, t251: F, t631: F, t675: F, t7242: F, t898: F, t2371: F, t665: F) -> (F, F, F, F, F, F, F, F) {
    let t9861 = t9723 / F::cast_from(9.0_f64);
    let t9862 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9727;
    let t9869 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9730;
    let t9870 = t9520 / F::cast_from(3.0_f64);
    let t9872 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9768;
    let t9876 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9765;
    let t9890 = F::cast_from(1.0_f64) / t251 / t631 / t898 / t675 / t7242 / F::cast_from(4.0_f64);
    let t9895 = t665 * t2371;
    (t9861, t9862, t9869, t9870, t9872, t9876, t9890, t9895)
}
