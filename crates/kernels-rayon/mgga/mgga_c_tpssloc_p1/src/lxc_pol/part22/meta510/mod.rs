//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1963;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1964;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1965;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1966;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta510(t1670: f64, t18258: f64, t3313: f64, t1694: f64, t6068: f64, t3403: f64, t11369: f64, t11372: f64, t14702: f64, t14766: f64, t18203: f64, t18219: f64, t18229: f64, t18494: f64, t18505: f64, t18512: f64, t21739: f64, t21741: f64, t21747: f64, t21751: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t21781: f64, t21783: f64, t21786: f64, t21789: f64, t21792: f64, t21795: f64, t21802: f64, t21804: f64, t1156: f64, t11285: f64, t1137: f64, t21854: f64, t1671: f64, t18686: f64, t4740: f64, t6021: f64, t14850: f64, t6024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21899, t21901, t21906) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1963(t1670, t18258, t3313, t1694, t6068);
        let (t21907, t21922) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1964(t21906, t3403, t11369, t11372, t14702, t14766, t18203, t18219, t18229, t18494, t18505, t18512, t21739, t21741, t21747, t21751);
        let t21937 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1965(t21760, t21764, t21767, t21771, t21774, t21778, t21781, t21783, t21786, t21789, t21792, t21795, t21802, t21804);
        let t21938 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1966(t21922, t21937);
        let (t21939, t21942, t21947, t21952, t21956, t21958, t21960) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1967(t1156, t21938, t11285, t21906, t1137, t21854, t1671, t18686, t4740, t6021, t14850, t6024);
    (t21899, t21901, t21906, t21907, t21938, t21939, t21942, t21947, t21952, t21956, t21958, t21960)
}
