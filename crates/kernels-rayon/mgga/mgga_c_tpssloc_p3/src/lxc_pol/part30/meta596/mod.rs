//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1978;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1979;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta596(t1336: f64, t22759: f64, t835: f64, t22760: f64, t3777: f64, t12248: f64, t6604: f64, t22716: f64, t6983: f64, t22723: f64, t268: f64, t534: f64, t22706: f64, t22863: f64, t6979: f64, t22641: f64, t3749: f64, t6978: f64, t80854: f64, t1984: f64, t80845: f64, t2010: f64, t6973: f64, t80742: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80997, t81000, t81027, t81039, t81046) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1978(t1336, t22759, t835, t22760, t3777, t12248, t6604, t22716, t6983, t22723, t268, t534);
        let (t81047, t81061, t81064, t81066, t81071, t81073, t81074) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1979(t22706, t81046, t22863, t6979, t22641, t3749, t6978, t80854, t1984, t80845, t2010, t6973, t80742);
    (t80997, t81000, t81027, t81039, t81046, t81047, t81061, t81064, t81066, t81071, t81073, t81074)
}
