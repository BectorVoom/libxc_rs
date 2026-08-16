//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta251 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1218;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1219;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1220;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1221;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1222;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta251(t1059: f64, t6800: f64, t6799: f64, t1049: f64, t1948: f64, t345: f64, t1022: f64, t1945: f64, t1060: f64, t383: f64, t6768: f64, t1003: f64, t1058: f64, t1920: f64, t1950: f64, t1953: f64, t353: f64, t6680: f64, t6687: f64, t6783: f64, t6787: f64, t6790: f64, t6797: f64, t1055: f64, t1052: f64, t1066: f64, t1923: f64, t1956: f64, t3026: f64, t3169: f64, t388: f64, t6685: f64, t6692: f64, t6695: f64, t6700: f64, t6707: f64, t6710: f64, t6769: f64, t6771: f64, t6776: f64, t1958: f64, t3216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6801, t6802) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1218(t1059, t6800, t6799);
        let t6805 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1219(t1049, t1948);
        let (t6806, t6811, t6813, t6815) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1220(t345, t6805, t1022, t1945, t1060, t383, t6768, t1003, t1058, t1920, t1950, t1953, t353, t6680, t6687, t6783, t6787, t6790, t6797, t6802);
        let t6816 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1221(t1055, t6815);
        let t6818 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1222(t1052, t1066, t1920, t1923, t1956, t3026, t3169, t388, t6680, t6685, t6687, t6692, t6695, t6700, t6707, t6710, t6769, t6771, t6776, t6816);
        let t6822 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1223(t1958, t3216);
    (t6801, t6802, t6805, t6806, t6811, t6813, t6815, t6816, t6818, t6822)
}
