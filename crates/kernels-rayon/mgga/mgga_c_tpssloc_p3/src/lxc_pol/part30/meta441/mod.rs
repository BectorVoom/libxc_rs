//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1687;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1688;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1689;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1690;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta441(t2240: f64, t608: f64, t1864: f64, t645: f64, t1863: f64, t6489: f64, t9231: f64, t192: f64, t532: f64, t1982: f64, t6995: f64, t2018: f64, t531: f64, t1887: f64, t6916: f64, t213: f64, t225: f64, t562: f64, t154: f64, t835: f64, t3748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22549, t22550, t22551, t22554, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1687(t2240, t608, t1864, t645, t1863, t6489, t9231, t192, t532, t1982);
        let (t22591, t22595, t22633) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1688(t532, t6995, t2018, t531, t1887, t6916);
        let t22635 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1689(t213, t225, t562);
        let t22641 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1690(t154, t835);
        let t22642 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1691(t22641, t3748);
    (t22549, t22550, t22551, t22554, t22573, t22574, t22591, t22595, t22633, t22635, t22641, t22642)
}
