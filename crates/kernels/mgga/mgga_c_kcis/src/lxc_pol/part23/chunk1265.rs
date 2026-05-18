//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1265/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1265<F: Float>(t1307: F, t28388: F, t52697: F, t94626: F, t98146: F, t98466: F, t98637: F, t98640: F, t98643: F, t98646: F, t98649: F, t98652: F, t98653: F, t98663: F, t98666: F) -> F {
    let t98668 = F::new(0.185671721767578125e-4) * t28388 * t98146 + F::new(0.22109259259259259258e-2) * t98637 - F::new(0.33163888888888888888e-2) * t98640 + F::new(0.13265555555555555555e-1) * t98643 - F::new(0.88437037037037037034e-2) * t98646 + t98649 + t98652 - F::new(0.92673611111111111113e-3) * t94626 * t98653 * t52697 * t1307 - F::new(0.46336805555555555556e-3) * t94626 * t98466 + F::new(0.66327777777777777776e-2) * t98663 - F::new(0.22109259259259259258e-2) * t98666;
    t98668
}
