//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 836/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk836<F: Float>(t12574: F, t12577: F, t12580: F, t12584: F, t12589: F, t12592: F, t12921: F, t12925: F, t12928: F, t13123: F, t9390: F, t13104: F, t13114: F, t13122: F) -> F {
    let t13133 = -t13123 + F::new(3.0) / F::new(8.0) * t12921 - t12925 / F::new(2.0) - t12928 / F::new(4.0) + F::new(2.0) / F::new(3.0) * t12574 + F::new(8.0) / F::new(3.0) * t12577 - F::new(2.0) / F::new(9.0) * t12580 + F::new(2.0) * t12584 - F::new(6.0) * t12589 + F::new(4.0) / F::new(9.0) * t12592 - t9390;
    let t13135 = t13104 + t13114 + t13122 + t13133;
    t13135
}
