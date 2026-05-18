//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 941/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk941<F: Float>(t12137: F, t1643: F, t2258: F, t2259: F, t2265: F, t2266: F, t2282: F, t2294: F, t358: F, t3613: F, t3621: F, t363: F, t37264: F, t37269: F, t37311: F, t37362: F, t39483: F, t39485: F, t39487: F, t39495: F, t39514: F, t39524: F, t39550: F, t39568: F, t631: F, t637: F, t639: F, t643: F, t7955: F, t7959: F, t7973: F, t8618: F, t8619: F, t8654: F) -> F {
    let t39574 = F::new(8.0) / F::new(9.0) * t39483 - F::new(8.0) * t39485 - F::new(16.0) / F::new(27.0) * t2265 * t39487 * t7955 * t643 + F::new(4.0) / F::new(9.0) * t2265 * t12137 * t37311 + F::new(2.0) * t2265 * t39495 * t1643 * t2282 + F::new(8.0) / F::new(3.0) * t2265 * t8654 * t7959 * t643 - F::new(4.0) / F::new(3.0) * t2265 * t2266 * t7973 * t643 - F::new(4.0) / F::new(3.0) * t2265 * t3621 * t37264 + F::new(2.0) / F::new(9.0) * t2265 * t3613 * t37269 - F::new(16.0) * t2265 * t39514 * t8619 * t358 * t363 + t631 * t2258 * t2259 * t37362 / F::new(6.0) - F::new(10.0) * t39524 + F::new(36.0) * t631 * t637 * t8618 * t2282 * t2294 + t631 * t637 * t639 * (t39550 + t39568) / F::new(2.0);
    t39574
}
