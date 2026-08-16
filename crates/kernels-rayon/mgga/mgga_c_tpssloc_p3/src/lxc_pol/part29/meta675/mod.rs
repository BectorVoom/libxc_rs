//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2264;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta675(t2018: f64, t40611: f64, t1845: f64, t3698: f64, t26161: f64, t15868: f64, t1983: f64, t6996: f64, t3734: f64, t24995: f64, t8643: f64, t23831: f64, t7458: f64, t22480: f64, t7461: f64, t9348: f64, t1774: f64, t22479: f64, t652: f64, t7468: f64, t15904: f64, t22574: f64, t33136: f64, t12734: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91690, t91694, t91698, t91704) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2264(t2018, t40611, t1845, t3698, t26161, t15868, t1983, t6996, t3734, t24995, t8643, t23831, t7458);
        let (t91706, t91708, t91713, t91715, t91718, t91722) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2265(t22480, t7458, t7461, t9348, t1774, t22479, t652, t7468, t15904, t22574, t33136, t12734);
    (t91690, t91694, t91698, t91704, t91706, t91708, t91713, t91715, t91718, t91722)
}
