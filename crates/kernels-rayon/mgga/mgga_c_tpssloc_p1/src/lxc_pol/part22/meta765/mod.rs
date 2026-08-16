//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2584;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2585;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2586;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta765(t1653: f64, t5011: f64, t19080: f64, t4997: f64, t1215: f64, t5398: f64, t11668: f64, t11678: f64, t11692: f64, t15569: f64, t15594: f64, t15659: f64, t1735: f64, t18236: f64, t18395: f64, t19016: f64, t22185: f64, t27524: f64, t3490: f64, t3577: f64, t3578: f64, t45119: f64, t4723: f64, t4729: f64, t475: f64, t52813: f64, t5971: f64, t6203: f64, t6230: f64, t6232: f64, t65424: f64, t65444: f64, t66388: f64, t19047: f64, t19040: f64, t5005: f64, t71095: f64, t71097: f64, t71106: f64, t71109: f64, t71112: f64, t71114: f64, t71118: f64, t71217: f64, t71221: f64, t71225: f64, t71227: f64, t71230: f64, t71233: f64, t71236: f64, t71238: f64, t71241: f64, t71245: f64, t71247: f64, t71249: f64, t71251: f64, t71255: f64, t71313: f64, t71315: f64, t71317: f64, t71319: f64, t71543: f64, t71545: f64, t71547: f64, t71655: f64, t71657: f64, t72045: f64, t72047: f64, t72050: f64, t72052: f64, t72058: f64, t72061: f64, t72065: f64, t72067: f64, t72071: f64, t72073: f64, t71697: f64, t71700: f64, t71704: f64, t71707: f64, t71711: f64, t71784: f64, t71786: f64, t71788: f64, t71790: f64, t71793: f64, t71795: f64, t71797: f64, t71800: f64, t71803: f64, t71806: f64, t71809: f64, t71811: f64, t71814: f64, t71817: f64, t71819: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t72146, t72180) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2584(t1653, t5011, t19080, t4997, t1215, t5398, t11668, t11678, t11692, t15569, t15594, t15659, t1735, t18236, t18395, t19016, t22185, t27524, t3490, t3577, t3578, t45119, t4723, t4729, t475, t52813, t5971, t6203, t6230, t6232, t65424, t65444, t66388);
        let (t72181, t72183, t72195) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2585(t19047, t4997, t19040, t5005, t71095, t71097, t71106, t71109, t71112, t71114, t71118, t71217, t71221, t71225, t71227, t71230, t71233, t71236, t71238, t71241, t71245, t71247, t71249, t71251);
        let t72196 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2586(t71255, t71313, t71315, t71317, t71319, t71543, t71545, t71547, t71655, t71657, t72045, t72047, t72050, t72052, t72058, t72061, t72065, t72067, t72071, t72073);
        let t72198 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2587(t71697, t71700, t71704, t71707, t71711, t71784, t71786, t71788, t71790, t71793, t71795, t71797, t71800, t71803, t71806, t71809, t71811, t71814, t71817, t71819);
    (t72146, t72180, t72181, t72183, t72195, t72196, t72198)
}
