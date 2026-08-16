//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1070;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1071;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta238<F: Float>(t3612: F, t6252: F, t1734: F, t1751: F, t1246: F, t491: F, t6218: F, t3625: F, t493: F, t6238: F, t1244: F, t1729: F, t1756: F, t1758: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F, t1241: F, t1238: F, t1761: F, t4945: F, t498: F, t5055: F, t6151: F, t6153: F, t6239: F, t6244: F, t1763: F, t1256: F, t193: F, t336: F, t3640: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6092: F, t6094: F, t6096: F, t6100: F, t6104: F, t6108: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1070::<F>(t3612, t6252, t1734, t1751, t1246, t491, t6218, t3625, t493, t6238, t1244, t1729, t1756, t1758, t3610, t3624, t470, t494, t5064, t6168);
        let (t6268, t6270, t6274) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1071::<F>(t1241, t6267, t1238, t1761, t4945, t498, t5055, t6151, t6153, t6239, t6244, t1763);
        let t6278 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1072::<F>(t1256, t193, t336, t3640, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108, t6270, t6274);
    (t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267, t6268, t6270, t6274, t6278)
}
