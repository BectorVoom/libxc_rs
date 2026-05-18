//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1264/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1264<F: Float>(t100656: F, t100660: F, t100666: F, t100669: F, t100672: F, t2192: F, t2197: F, t27042: F, t29094: F, t70078: F, t96121: F, t97265: F, t97273: F, t97281: F) -> F {
    let t100674 = F::new(0.37101880208333333333e-3) * t27042 * t29094 - F::new(0.46377350260416666667e-4) * t100656 + t97265 - F::new(0.51588271604938271603e-3) * t96121 - t97273 - F::new(0.92858888888888888885e-2) * t100660 - t97281 - F::new(0.34752604166666666667e-3) * t70078 * t2192 * t2197 + F::new(0.61905925925925925925e-2) * t100666 + F::new(0.46429444444444444444e-2) * t100669 + F::new(0.11607361111111111111e-2) * t100672;
    t100674
}
