//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1193/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1193<F: Float>(t13691: F, t13694: F, t16209: F, t16211: F, t16213: F, t21669: F, t21671: F, t21675: F, t21679: F, t21681: F, t21684: F, t21687: F, t21691: F, t21695: F) -> F {
    let t21700 = F::new(0.5868e1) * t21669 - F::new(0.3912e1) * t21671 - F::new(0.22005e1) * t21675 + F::new(0.1467e1) * t21679 - F::new(0.1956e1) * t21681 + F::new(0.1467e1) * t21684 + F::new(0.7335e0) * t21687 + F::new(0.8802e1) * t21691 - F::new(0.22005e1) * t21695 + F::new(0.978e0) * t16209 - F::new(0.4564e1) * t16211 + F::new(0.76066666666666666667e1) * t16213 - t13691 + t13694;
    t21700
}
