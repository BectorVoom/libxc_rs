//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk691;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk692;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk693;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk694;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk695;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk696;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk697;
use chunk7::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk698;
use chunk8::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta121(t261: f64, t193: f64, t202: f64, t2486: f64, t2522: f64, t2523: f64, t2530: f64, t2533: f64, t2537: f64, t2539: f64, t2553: f64, t2654: f64, t2657: f64, t2661: f64, t2665: f64, t2745: f64, t2749: f64, t766: f64, t776: f64, t870: f64, t2521: f64, t1878: f64, t268: f64, t271: f64, t690: f64, t885: f64, t1043: f64, t154: f64, t632: f64, t2244: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2751, t2752) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk691(t261);
        let t2755 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk692(t193, t202, t2486, t2522, t2523, t2530, t2533, t2537, t2539, t2553, t2654, t2657, t2661, t2665, t2745, t2749, t2752, t766, t776, t870);
        let t2756 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk693(t2521, t2755);
        let t2764 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk694(t1878, t268, t271);
        let (t2765, t2766) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk695(t2764, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk696(t1043, t154);
        let (t2769, t2770) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk697(t632);
        let t2771 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk698(t2244, t2770);
        let (t2772, t2773) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk699(t2768, t2771, t123);
    (t2751, t2752, t2756, t2764, t2765, t2766, t2768, t2769, t2770, t2771, t2772, t2773)
}
