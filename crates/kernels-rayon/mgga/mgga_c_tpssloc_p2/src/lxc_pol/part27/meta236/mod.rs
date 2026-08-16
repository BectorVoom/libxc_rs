//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1126;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1127;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1128;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1129;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1130;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1131;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1132;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta236(t1458: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t4072: f64, t5363: f64, t5371: f64, t577: f64, t2235: f64, t33: f64, t1862: f64, t2240: f64, t645: f64, t79: f64, t72: f64, t605: f64, t608: f64, t38: f64, t43: f64, t625: f64, t44: f64, t607: f64, t614: f64, t67: f64, t1864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5376, t5381, t6486) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1126(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2235, t33);
        let t6489 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1127(t1862, t33);
        let t6490 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1128(t2240, t6489);
        let t6492 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1129(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1130(t605, t608);
        let t6500 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1131(t38, t43);
        let (t6503, t6504, t6505) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1132(t625, t44, t607, t614, t6500, t67);
        let t6506 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1133(t1864, t6505);
    (t5376, t5381, t6486, t6489, t6490, t6492, t6495, t6500, t6503, t6504, t6505, t6506)
}
