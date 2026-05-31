//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1092/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1092<F: Float>(t13710: F, t13713: F, t13715: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F, t9736: F) -> F {
    let t13744 = -t9736 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9691 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9683 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9700 + t9681 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13710 + t13713 - t13715 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t13717 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t13720 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13723 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13726 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13729 - F::cast_from(2.0_f64) * t13732 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t13735 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13738 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13742;
    t13744
}
