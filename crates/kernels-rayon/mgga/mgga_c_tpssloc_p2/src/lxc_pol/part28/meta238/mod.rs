//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1042;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1043;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1044;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta238(t1458: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t4072: f64, t5363: f64, t5371: f64, t577: f64, t2235: f64, t33: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t641: f64, t71: f64, t107: f64, t625: f64, t63: f64, t656: f64, t666: f64, t25: f64, t776: f64, t154: f64, t781: f64, t1879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5376, t5381, t6486) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1039(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2235, t33);
        let t6492 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1040(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1041(t605, t608);
        let t6509 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1042(t641, t71);
        let (t6528, t6530) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1043(t107, t625, t63, t656);
        let (t6531, t6542, t6546) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1044(t6530, t666, t25, t776, t154, t781);
        let t6547 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1045(t1879, t6546);
    (t5376, t5381, t6486, t6492, t6495, t6509, t6528, t6530, t6531, t6542, t6546, t6547)
}
