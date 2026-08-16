//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta678 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2270;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2271;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2272;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2273;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2274;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2275;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta678<F: Float>(t2363: F, t3941: F, t7467: F, t12724: F, t12728: F, t16503: F, t2165: F, t2167: F, t2364: F, t24552: F, t27858: F, t27863: F, t4028: F, t4072: F, t650: F, t652: F, t7408: F, t7989: F, t86673: F, t86676: F, t86679: F, t86682: F, t86684: F, t86688: F, t86693: F, t86698: F, t86700: F, t86702: F, t90020: F, t9348: F, t12725: F, t12734: F, t1442: F, t1458: F, t2314: F, t24924: F, t27879: F, t4026: F, t7271: F, t90022: F, t90026: F, t90029: F, t90034: F, t90036: F, t90038: F, t90040: F, t90051: F, t90059: F, t90062: F, t90064: F, t90068: F, t90418: F, t2113: F, t12557: F, t1459: F, t1774: F, t24543: F, t24545: F, t24932: F, t27888: F, t4037: F, t4073: F, t7266: F, t8103: F, t85428: F, t90421: F, t90428: F, t90434: F, t90436: F, t90440: F, t90444: F, t90447: F, t90450: F, t90454: F, t90456: F, t2319: F, t7982: F, t12550: F, t1266: F, t12841: F, t24935: F, t27290: F, t27371: F, t3652: F, t4034: F, t510: F, t7983: F, t91564: F, t91568: F, t91570: F, t91573: F, t91578: F, t91580: F, t91582: F, t91585: F, t91587: F, t91589: F, t91591: F, t91593: F, t25: F, t265: F, t394: F, t89823: F, t12606: F, t1409: F, t2116: F, t2250: F, t24555: F, t27373: F, t3966: F, t40: F, t607: F, t7274: F, t7992: F, t88003: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1751: F, t7319: F, t1240: F, t5088: F, t1089: F, t3597: F, t1090: F, t12648: F, t14165: F, t24589: F, t24601: F, t24883: F, t24887: F, t27381: F, t27444: F, t27445: F, t27549: F, t27774: F, t27775: F, t27820: F, t3248: F, t3252: F, t3599: F, t7287: F, t8002: F, t85640: F, t85648: F, t86415: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91802, t94223) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2270::<F>(t2363, t3941, t7467, t12724, t12728, t16503, t2165, t2167, t2364, t24552, t27858, t27863, t4028, t4072, t650, t652, t7408, t7989, t86673, t86676, t86679, t86682, t86684, t86688, t86693, t86698, t86700, t86702, t90020, t9348);
        let t94236 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2271::<F>(t12725, t12734, t1442, t1458, t2314, t24924, t27879, t4026, t652, t7271, t7408, t7989, t90022, t90026, t90029, t90034, t90036, t90038, t90040, t90051, t90059, t90062, t90064, t90068, t90418);
        let (t94248, t94257) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2272::<F>(t2113, t2363, t12557, t1459, t1774, t24543, t24545, t24932, t27888, t4028, t4037, t4073, t652, t7266, t8103, t85428, t90421, t90428, t90434, t90436, t90440, t90444, t90447, t90450, t90454, t90456);
        let (t94265, t94272) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2273::<F>(t2319, t7982, t12550, t1266, t12841, t1774, t24935, t27290, t27371, t3652, t4034, t510, t7266, t7983, t91564, t91568, t91570, t91573, t91578, t91580, t91582, t91585, t91587, t91589, t91591, t91593);
        let t94293 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2274::<F>(t25, t265, t394, t89823, t12606, t1409, t2116, t2250, t24555, t27373, t3966, t40, t607, t7274, t7992, t88003, dens_threshold, rho0, zeta_threshold);
        let (t94319, t94341) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2275::<F>(t1751, t7319, t1240, t5088, t1089, t3597, t1090, t12648, t1409, t14165, t24589, t24601, t24883, t24887, t27381, t27444, t27445, t27549, t27774, t27775, t27820, t3248, t3252, t3599, t7287, t8002, t85640, t85648, t86415);
    (t91802, t94223, t94236, t94248, t94257, t94265, t94272, t94293, t94319, t94341)
}
