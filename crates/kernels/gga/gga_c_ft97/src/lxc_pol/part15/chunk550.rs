//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 550/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk550<F: Float>(t1557: F, t81: F, t1570: F, t360: F, t18: F, t23: F, t7241: F, t174: F, t358: F) -> (F, F, F, F, F, F, F) {
    let t7712 = t81 * t1557;
    let t7720 = t81 * t1570;
    let t7741 = t360 * t360;
    let t7742 = F::new(1.0) / t7741;
    let t7743 = t18 * t7742;
    let t7750 = t23 * t7241;
    let t7760 = F::new(1.0) / t174 / t358;
    (t7712, t7720, t7741, t7742, t7743, t7750, t7760)
}
