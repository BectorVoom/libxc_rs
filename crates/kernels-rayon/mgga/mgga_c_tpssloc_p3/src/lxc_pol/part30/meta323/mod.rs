//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1348;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta323(t1090: f64, t11789: f64, t248: f64, t1227: f64, t486: f64, t676: f64, t1216: f64, t1213: f64, t11552: f64, t221: f64, t456: f64, t1197: f64, t698: f64, t1174: f64, t10471: f64, t11715: f64, t11712: f64, t11721: f64, t6739: f64, t3502: f64, t3508: f64, t11707: f64, t3609: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11792, t11818, t11821, t11834, t11835) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1348(t1090, t11789, t248, t1227, t486, t676, t1216, t1213, t11552, t221, t456, t1197, t698);
        let (t11836, t11881, t11883, t11888, t11889, t11904, t11907) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1349(t1174, t11835, t10471, t11715, t11712, t11721, t6739, t3502, t3508, t11707, t3609, t3623);
    (t11792, t11818, t11821, t11834, t11836, t11881, t11883, t11888, t11889, t11904, t11907)
}
