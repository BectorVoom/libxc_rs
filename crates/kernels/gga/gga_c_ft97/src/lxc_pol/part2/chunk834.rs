//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 834/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk834<F: Float>(t12327: F, t12319: F, t12322: F, t12325: F, t12332: F, t12336: F, t12340: F, t8796: F, t9065: F, t9366: F, t9370: F, t12356: F) -> (F, F) {
    let t13108 = F::new(2.0) / F::new(9.0) * t12327;
    let t13114 = -F::new(2.0) / F::new(3.0) * t12319 - F::new(2.0) * t12322 + F::new(4.0) / F::new(3.0) * t12325 - t13108 + F::new(2.0) / F::new(3.0) * t12332 - F::new(4.0) / F::new(3.0) * t12336 - F::new(4.0) / F::new(3.0) * t12340 - t9366 - F::new(8.0) / F::new(9.0) * t9065 + t9370 - F::new(8.0) / F::new(27.0) * t8796;
    let t13117 = F::new(4.0) / F::new(3.0) * t12356;
    (t13114, t13117)
}
