//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1419;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1420;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1421;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1422;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta380(t11277: f64, t3307: f64, t11275: f64, t3265: f64, t11400: f64, t11628: f64, t1164: f64, t11285: f64, t3395: f64, t11282: f64, t3377: f64, t11403: f64, t11424: f64, t43924: f64, t43953: f64, t43956: f64, t43958: f64, t43961: f64, t43963: f64, t43966: f64, t43973: f64, t43975: f64, t3266: f64, t3313: f64, t1119: f64, t11269: f64, t3264: f64, t11190: f64, t3316: f64, t11185: f64, t11407: f64, t1117: f64, t3315: f64, t43713: f64, t43717: f64, t43721: f64, t43725: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43748: f64, t43750: f64, t43754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43979, t43982, t43984, t43987, t43989) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1419(t11277, t3307, t11275, t3265, t11400, t11628, t1164, t11285, t3395, t11282, t3377, t11403, t11424);
        let t43990 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1420(t43924, t43953, t43956, t43958, t43961, t43963, t43966, t43973, t43975, t43979, t43982, t43987, t43989);
        let (t43994, t43997, t44000, t44002, t44006) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1421(t3266, t3307, t3313, t1119, t11269, t3264, t11190, t3316, t11185, t11407, t1117, t3315);
        let t44021 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1422(t43713, t43717, t43721, t43725, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43748, t43750, t43754);
    (t43979, t43982, t43984, t43987, t43989, t43990, t43994, t43997, t44000, t44002, t44006, t44021)
}
