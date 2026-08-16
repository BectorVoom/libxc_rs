//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2509;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2510;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2511;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta667(t14783: f64, t699: f64, t14786: f64, t14789: f64, t50946: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t136: f64, t43761: f64, t50924: f64, t14778: f64, t11219: f64, t50910: f64, t50915: f64, t11153: f64, t1229: f64, t45971: f64, t47774: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t43895: f64, t3242: f64, t486: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50968, t50970, t50972, t50974) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2509(t14783, t699, t14786, t14789, t50946, t50948, t50950, t50952, t50954, t50957, t50961, t50966);
        let (t50976, t50978, t50987, t50990, t50992, t50994) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2510(t136, t43761, t50924, t14778, t699, t11219, t50910, t50915, t11153, t1229, t45971, t47774);
        let t50996 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2511(t43780, t43782, t43784, t43786, t43788, t43816, t43895, t50976, t50978, t50987, t50990, t50994);
        let t51000 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2512(t3242, t486, t45971, t47774);
    (t50968, t50970, t50972, t50974, t50976, t50978, t50987, t50990, t50992, t50994, t50996, t51000)
}
