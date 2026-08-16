//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1065;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1066;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1067;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1068;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1069;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta236(t466: f64, t6238: f64, t1760: f64, t3598: f64, t491: f64, t6224: f64, t3612: f64, t1734: f64, t1751: f64, t1246: f64, t6218: f64, t3625: f64, t493: f64, t1244: f64, t1729: f64, t1756: f64, t1758: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t5064: f64, t6168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6239, t6243) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1065(t466, t6238, t1760);
        let t6244 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1066(t3598, t6243);
        let t6252 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1067(t491, t6224);
        let (t6253, t6256) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1068(t3612, t6252, t1734, t1751);
        let (t6257, t6260) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1069(t1246, t6256, t491, t6218);
        let (t6261, t6263, t6265, t6267) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1070(t1246, t6260, t3625, t6252, t493, t6238, t1244, t1729, t1756, t1758, t3610, t3624, t470, t494, t5064, t6168, t6253, t6257);
    (t6239, t6243, t6244, t6252, t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267)
}
