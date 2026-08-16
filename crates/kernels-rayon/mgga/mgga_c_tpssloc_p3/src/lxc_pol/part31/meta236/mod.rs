//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk985;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk986;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk987;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta236(t3612: f64, t6252: f64, t1734: f64, t1751: f64, t1246: f64, t491: f64, t6218: f64, t3625: f64, t493: f64, t6238: f64, t1244: f64, t1729: f64, t1756: f64, t1758: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t5064: f64, t6168: f64, t1241: f64, t1238: f64, t1761: f64, t4945: f64, t498: f64, t5055: f64, t6151: f64, t6153: f64, t6239: f64, t6244: f64, t1763: f64, t1256: f64, t193: f64, t336: f64, t3640: f64, t5985: f64, t5987: f64, t5991: f64, t6023: f64, t6026: f64, t6092: f64, t6094: f64, t6096: f64, t6100: f64, t6104: f64, t6108: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk985(t3612, t6252, t1734, t1751, t1246, t491, t6218, t3625, t493, t6238, t1244, t1729, t1756, t1758, t3610, t3624, t470, t494, t5064, t6168);
        let (t6268, t6270, t6274) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk986(t1241, t6267, t1238, t1761, t4945, t498, t5055, t6151, t6153, t6239, t6244, t1763);
        let t6278 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk987(t1256, t193, t336, t3640, t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108, t6270, t6274);
    (t6253, t6256, t6257, t6260, t6261, t6263, t6265, t6267, t6268, t6270, t6274, t6278)
}
