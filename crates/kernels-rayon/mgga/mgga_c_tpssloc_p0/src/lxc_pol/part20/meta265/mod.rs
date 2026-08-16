//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1416;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1417;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1418;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1419;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1420;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta265(t2830: f64, t699: f64, t2833: f64, t241: f64, t2978: f64, t10216: f64, t9288: f64, t136: f64, t10277: f64, t2826: f64, t10195: f64, t2770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10300 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1416(t2830, t699);
        let t10302 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1417(t2833, t699);
        let (t10304, t10305) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1418(t241, t2978, t10216, t9288);
        let (t10306, t10307, t10309) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1419(t10304, t10305, t136, t10277, t9288);
        let (t10310, t10311, t10313, t10314, t10316) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1420(t10309, t2826, t136, t10195, t2770, t9288);
    (t10300, t10302, t10304, t10305, t10306, t10307, t10309, t10310, t10311, t10313, t10314, t10316)
}
