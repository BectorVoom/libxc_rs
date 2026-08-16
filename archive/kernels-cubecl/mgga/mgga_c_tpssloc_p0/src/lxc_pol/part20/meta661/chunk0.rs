//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2476/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2476<F: Float>(t265: F, t394: F, t49244: F, t49256: F, t49259: F, t49262: F, t49268: F, t49271: F, t49273: F, t49276: F, t49567: F, t49572: F, t49575: F, t47655: F, t49585: F, t50750: F, t50755: F, t50757: F, t50764: F, t50771: F, t50779: F) -> F {
    let t395 = t265 < t394;
    let t50781 = t49244 - t49567 - t49572 + t49256 + t49259 + t49262 - t49575 + t49268 + t49271 + t49273 + t49276;
    let t50785 = piecewise3::<F>(t395, t50750 + t50755 + t50757 + t50764 + t50771 + t50779 + t50781 + t49585, t47655);
    t50785
}
