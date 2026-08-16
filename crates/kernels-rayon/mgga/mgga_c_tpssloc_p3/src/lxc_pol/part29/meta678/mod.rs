//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta678 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2270;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2271;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2272;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2273;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2274;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2275;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta678(t2363: f64, t3941: f64, t7467: f64, t12724: f64, t12728: f64, t16503: f64, t2165: f64, t2167: f64, t2364: f64, t24552: f64, t27858: f64, t27863: f64, t4028: f64, t4072: f64, t650: f64, t652: f64, t7408: f64, t7989: f64, t86673: f64, t86676: f64, t86679: f64, t86682: f64, t86684: f64, t86688: f64, t86693: f64, t86698: f64, t86700: f64, t86702: f64, t90020: f64, t9348: f64, t12725: f64, t12734: f64, t1442: f64, t1458: f64, t2314: f64, t24924: f64, t27879: f64, t4026: f64, t7271: f64, t90022: f64, t90026: f64, t90029: f64, t90034: f64, t90036: f64, t90038: f64, t90040: f64, t90051: f64, t90059: f64, t90062: f64, t90064: f64, t90068: f64, t90418: f64, t2113: f64, t12557: f64, t1459: f64, t1774: f64, t24543: f64, t24545: f64, t24932: f64, t27888: f64, t4037: f64, t4073: f64, t7266: f64, t8103: f64, t85428: f64, t90421: f64, t90428: f64, t90434: f64, t90436: f64, t90440: f64, t90444: f64, t90447: f64, t90450: f64, t90454: f64, t90456: f64, t2319: f64, t7982: f64, t12550: f64, t1266: f64, t12841: f64, t24935: f64, t27290: f64, t27371: f64, t3652: f64, t4034: f64, t510: f64, t7983: f64, t91564: f64, t91568: f64, t91570: f64, t91573: f64, t91578: f64, t91580: f64, t91582: f64, t91585: f64, t91587: f64, t91589: f64, t91591: f64, t91593: f64, t25: f64, t265: f64, t394: f64, t89823: f64, t12606: f64, t1409: f64, t2116: f64, t2250: f64, t24555: f64, t27373: f64, t3966: f64, t40: f64, t607: f64, t7274: f64, t7992: f64, t88003: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1751: f64, t7319: f64, t1240: f64, t5088: f64, t1089: f64, t3597: f64, t1090: f64, t12648: f64, t14165: f64, t24589: f64, t24601: f64, t24883: f64, t24887: f64, t27381: f64, t27444: f64, t27445: f64, t27549: f64, t27774: f64, t27775: f64, t27820: f64, t3248: f64, t3252: f64, t3599: f64, t7287: f64, t8002: f64, t85640: f64, t85648: f64, t86415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91802, t94223) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2270(t2363, t3941, t7467, t12724, t12728, t16503, t2165, t2167, t2364, t24552, t27858, t27863, t4028, t4072, t650, t652, t7408, t7989, t86673, t86676, t86679, t86682, t86684, t86688, t86693, t86698, t86700, t86702, t90020, t9348);
        let t94236 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2271(t12725, t12734, t1442, t1458, t2314, t24924, t27879, t4026, t652, t7271, t7408, t7989, t90022, t90026, t90029, t90034, t90036, t90038, t90040, t90051, t90059, t90062, t90064, t90068, t90418);
        let (t94248, t94257) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2272(t2113, t2363, t12557, t1459, t1774, t24543, t24545, t24932, t27888, t4028, t4037, t4073, t652, t7266, t8103, t85428, t90421, t90428, t90434, t90436, t90440, t90444, t90447, t90450, t90454, t90456);
        let (t94265, t94272) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2273(t2319, t7982, t12550, t1266, t12841, t1774, t24935, t27290, t27371, t3652, t4034, t510, t7266, t7983, t91564, t91568, t91570, t91573, t91578, t91580, t91582, t91585, t91587, t91589, t91591, t91593);
        let t94293 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2274(t25, t265, t394, t89823, t12606, t1409, t2116, t2250, t24555, t27373, t3966, t40, t607, t7274, t7992, t88003, dens_threshold, rho0, zeta_threshold);
        let (t94319, t94341) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2275(t1751, t7319, t1240, t5088, t1089, t3597, t1090, t12648, t1409, t14165, t24589, t24601, t24883, t24887, t27381, t27444, t27445, t27549, t27774, t27775, t27820, t3248, t3252, t3599, t7287, t8002, t85640, t85648, t86415);
    (t91802, t94223, t94236, t94248, t94257, t94265, t94272, t94293, t94319, t94341)
}
