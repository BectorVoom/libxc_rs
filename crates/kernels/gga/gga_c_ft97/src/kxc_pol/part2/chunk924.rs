//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 924/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk924<F: Float>(t13704: F, t13708: F, t13719: F, t13722: F, t13728: F, t13732: F, t13736: F, t13739: F, t13743: F, t13750: F, t14317: F, t14318: F, t14327: F, t14329: F, t9520: F, t9723: F, t9727: F, t9730: F, t9765: F, t9768: F) -> F {
    let t14332 = -F::new(2.0) / F::new(9.0) * t13704 + F::new(2.0) / F::new(27.0) * t13708 + t9723 / F::new(54.0) + t9727 / F::new(81.0) - t14317 - t14318 - t9730 / F::new(9.0) + t9520 / F::new(18.0) - t13719 - F::new(2.0) / F::new(81.0) * t13722 + F::new(2.0) / F::new(3.0) * t13728 - F::new(11.0) / F::new(27.0) * t13732 + t13736 / F::new(9.0) - F::new(2.0) / F::new(27.0) * t13739 + t13743 / F::new(3.0) - t14327 - t13750 / F::new(6.0) + t14329 - t9768 / F::new(27.0) - t9765 / F::new(27.0);
    t14332
}
