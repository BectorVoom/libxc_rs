//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta58 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk375;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk376;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta58(t1089: f64, t461: f64, t607: f64, t1177: f64, t1111: f64, t1115: f64, t457: f64, t460: f64, t974: f64, t1173: f64, t1174: f64, t491: f64, t1169: f64, t221: f64, t456: f64, t1176: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1178, t1179, t1180, t1184) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk375(t1089, t461, t607, t1177, t1111, t1115);
        let (t1186, t1190) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk376(t1184, t457, t460, t974, t1173, t1174, t1180);
        let (t1191, t1195, t1196, t1197, t1198, t1201) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk377(t1190, t491, t1169, t221, t456, t1089, t1176, t607, t974, t225);
    (t1178, t1179, t1184, t1186, t1190, t1191, t1195, t1196, t1197, t1198, t1201)
}
