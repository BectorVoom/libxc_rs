//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1092/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1092<F: Float>(t13710: F, t13713: F, t13715: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F, t9736: F) -> F {
    let t13744 = -t9736 - F::new(8.0) / F::new(27.0) * t9691 + F::new(2.0) / F::new(27.0) * t9683 - F::new(2.0) / F::new(9.0) * t9700 + t9681 / F::new(9.0) - F::new(4.0) / F::new(27.0) * t13710 + t13713 - t13715 + F::new(22.0) / F::new(9.0) * t13717 - F::new(10.0) / F::new(27.0) * t13720 + F::new(4.0) / F::new(3.0) * t13723 - F::new(8.0) / F::new(9.0) * t13726 - F::new(2.0) / F::new(9.0) * t13729 - F::new(2.0) * t13732 + F::new(8.0) / F::new(3.0) * t13735 + F::new(2.0) / F::new(3.0) * t13738 - F::new(2.0) / F::new(3.0) * t13742;
    t13744
}
