//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta721 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2565;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2566;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2567;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta721(t14536: f64, t225: f64, t10164: f64, t1634: f64, t14532: f64, t14562: f64, t14527: f64, t14534: f64, t11190: f64, t1670: f64, t3242: f64, t457: f64, t2394: f64, t4734: f64, t14707: f64, t690: f64, t1654: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50625, t50628, t50632, t50653, t50690, t50703, t50819, t50822) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2565(t14536, t225, t10164, t1634, t14532, t14562, t14527, t14534, t11190, t1670, t3242, t457);
        let t50826 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2566(t2394, t4734);
        let t50828 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2567(t14707, t690);
        let t50834 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2568(t1654, t9698);
    (t50625, t50628, t50632, t50653, t50690, t50703, t50819, t50822, t50826, t50828, t50834)
}
