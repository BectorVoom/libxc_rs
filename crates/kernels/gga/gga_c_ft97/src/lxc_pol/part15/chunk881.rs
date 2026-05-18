//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 881/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk881<F: Float>(t2: F, t33300: F, t626: F, t703: F, t240: F, t9577: F, t342: F, t657: F, t8639: F, t9570: F, t762: F, t9895: F) -> (F, F, F, F, F, F) {
    let t42218 = t33300 * t2;
    let t42262 = t626 * t703;
    let t42279 = t240 * t9577;
    let t42293 = F::new(5.0) / F::new(54.0) * t342 * t8639 * t657;
    let t42307 = t240 * t9570;
    let t42334 = t9895 * t762;
    (t42218, t42262, t42279, t42293, t42307, t42334)
}
