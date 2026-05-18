//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1233/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1233<F: Float>(t13696: F, t13699: F, t13701: F, t13706: F, t13714: F, t13729: F, t13810: F, t13812: F, t16230: F, t21707: F, t21709: F, t21712: F, t21714: F, t21717: F) -> F {
    let t22575 = -F::new(40.0) / F::new(27.0) * t13696 + F::new(4.0) / F::new(3.0) * t13699 + t13701 / F::new(6.0) + t13706 / F::new(6.0) - t13714 / F::new(12.0) + t13810 - t13729 / F::new(3.0) + t13812 + F::new(2.0) / F::new(3.0) * t21707 + F::new(14.0) / F::new(9.0) * t21709 + t21712 - F::new(7.0) / F::new(9.0) * t21714 - t21717 / F::new(4.0) + F::new(2.0) / F::new(3.0) * t16230;
    t22575
}
