//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2476/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2476(t265: f64, t394: f64, t49244: f64, t49256: f64, t49259: f64, t49262: f64, t49268: f64, t49271: f64, t49273: f64, t49276: f64, t49567: f64, t49572: f64, t49575: f64, t47655: f64, t49585: f64, t50750: f64, t50755: f64, t50757: f64, t50764: f64, t50771: f64, t50779: f64) -> f64 {
    let t395 = t265 < t394;
    let t50781 = t49244 - t49567 - t49572 + t49256 + t49259 + t49262 - t49575 + t49268 + t49271 + t49273 + t49276;
    let t50785 = piecewise3(t395, t50750 + t50755 + t50757 + t50764 + t50771 + t50779 + t50781 + t49585, t47655);
    t50785
}
