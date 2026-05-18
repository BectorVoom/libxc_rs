//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 805/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk805<F: Float>(t12610: F, t12614: F, t12617: F, t12620: F, t12622: F, t12626: F, t12630: F, t12634: F, t12638: F, t12642: F, t12644: F, t12647: F, t12652: F, t12656: F, t12660: F, t1901: F, t446: F) -> F {
    let t12663 = -F::new(2.0) / F::new(9.0) * t1901 * t12610 + F::new(2.0) / F::new(3.0) * t446 * t12614 - F::new(4.0) / F::new(81.0) * t12617 + t12620 - F::new(2.0) / F::new(9.0) * t446 * t12622 - F::new(2.0) / F::new(9.0) * t1901 * t12626 - F::new(2.0) / F::new(3.0) * t446 * t12630 - F::new(2.0) / F::new(3.0) * t446 * t12634 - t446 * t12638 / F::new(3.0) - t12642 - t12644 + F::new(4.0) / F::new(3.0) * t446 * t12647 + F::new(2.0) / F::new(3.0) * t446 * t12652 + F::new(4.0) / F::new(3.0) * t446 * t12656 + F::new(2.0) / F::new(3.0) * t446 * t12660;
    t12663
}
