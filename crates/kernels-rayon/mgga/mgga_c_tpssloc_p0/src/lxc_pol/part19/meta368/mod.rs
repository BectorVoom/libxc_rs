//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1356;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1357;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1358;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta368(t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64, t10510: f64, t3109: f64, t10309: f64, t10390: f64, t10398: f64, t10408: f64, t10410: f64, t10413: f64, t10419: f64, t10493: f64, t10858: f64, t10886: f64, t10937: f64, t2776: f64, t3041: f64, t3070: f64, t3071: f64, t3117: f64, t43186: f64, t43200: f64, t43206: f64, t43211: f64, t43214: f64, t884: f64, t10965: f64, t3053: f64, t3082: f64, t3094: f64, t10895: f64, t10952: f64, t1022: f64, t3120: f64, t2250: f64, t360: f64, t1036: f64, t10367: f64, t1032: f64, t10375: f64, t370: f64, t374: f64, t9697: f64, t10908: f64, t10446: f64, t1004: f64, t10249: f64, t10445: f64, t14220: f64, t2979: f64, t35: f64, t354: f64, t364: f64, t378: f64, t41649: f64, t6720: f64, t973: f64, t10997: f64, t135: f64, t10480: f64, t10483: f64, t3101: f64, t10876: f64, t10877: f64, t10883: f64, t10884: f64, t10473: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t43223 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1356(t204, t376, t1020, t1023, t248, t10510, t3109, t10309, t10390, t10398, t10408, t10410, t10413, t10419, t10493, t10858, t10886, t10937, t2776, t3041, t3070, t3071, t3117, t43186, t43200, t43206, t43211, t43214, t884);
        let (t43226, t43228, t43233, t43235, t43241, t43246) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1357(t10965, t3053, t3082, t3094, t10895, t10952, t1022, t3120, t2250, t360, t1036, t10367);
        let t43267 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1358(t1032, t10375, t370, t374, t376, t9697, t10908, t3109, t1036, t10446, t1004, t10249, t10413, t10445, t14220, t2979, t3070, t3071, t35, t354, t364, t378, t41649, t43226, t43228, t43233, t43235, t43241, t43246, t6720, t973);
        let (t43273, t43277, t43281, t43285, t43288) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1359(t10997, t135, t973, t10480, t10483, t248, t3101, t10876, t10877, t10883, t10884, t10473, t361);
    (t43223, t43267, t43273, t43277, t43281, t43285, t43288)
}
