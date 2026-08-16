//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1963;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1964;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1965;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1966;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta510<F: Float>(t1670: F, t18258: F, t3313: F, t1694: F, t6068: F, t3403: F, t11369: F, t11372: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F, t21741: F, t21747: F, t21751: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F, t21783: F, t21786: F, t21789: F, t21792: F, t21795: F, t21802: F, t21804: F, t1156: F, t11285: F, t1137: F, t21854: F, t1671: F, t18686: F, t4740: F, t6021: F, t14850: F, t6024: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21899, t21901, t21906) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1963::<F>(t1670, t18258, t3313, t1694, t6068);
        let (t21907, t21922) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1964::<F>(t21906, t3403, t11369, t11372, t14702, t14766, t18203, t18219, t18229, t18494, t18505, t18512, t21739, t21741, t21747, t21751);
        let t21937 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1965::<F>(t21760, t21764, t21767, t21771, t21774, t21778, t21781, t21783, t21786, t21789, t21792, t21795, t21802, t21804);
        let t21938 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1966::<F>(t21922, t21937);
        let (t21939, t21942, t21947, t21952, t21956, t21958, t21960) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1967::<F>(t1156, t21938, t11285, t21906, t1137, t21854, t1671, t18686, t4740, t6021, t14850, t6024);
    (t21899, t21901, t21906, t21907, t21938, t21939, t21942, t21947, t21952, t21956, t21958, t21960)
}
