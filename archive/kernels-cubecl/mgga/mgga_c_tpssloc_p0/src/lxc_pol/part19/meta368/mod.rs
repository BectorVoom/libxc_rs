//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1356;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1357;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1358;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta368<F: Float>(t204: F, t376: F, t1020: F, t1023: F, t248: F, t10510: F, t3109: F, t10309: F, t10390: F, t10398: F, t10408: F, t10410: F, t10413: F, t10419: F, t10493: F, t10858: F, t10886: F, t10937: F, t2776: F, t3041: F, t3070: F, t3071: F, t3117: F, t43186: F, t43200: F, t43206: F, t43211: F, t43214: F, t884: F, t10965: F, t3053: F, t3082: F, t3094: F, t10895: F, t10952: F, t1022: F, t3120: F, t2250: F, t360: F, t1036: F, t10367: F, t1032: F, t10375: F, t370: F, t374: F, t9697: F, t10908: F, t10446: F, t1004: F, t10249: F, t10445: F, t14220: F, t2979: F, t35: F, t354: F, t364: F, t378: F, t41649: F, t6720: F, t973: F, t10997: F, t135: F, t10480: F, t10483: F, t3101: F, t10876: F, t10877: F, t10883: F, t10884: F, t10473: F, t361: F) -> (F, F, F, F, F, F, F) {
        let t43223 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1356::<F>(t204, t376, t1020, t1023, t248, t10510, t3109, t10309, t10390, t10398, t10408, t10410, t10413, t10419, t10493, t10858, t10886, t10937, t2776, t3041, t3070, t3071, t3117, t43186, t43200, t43206, t43211, t43214, t884);
        let (t43226, t43228, t43233, t43235, t43241, t43246) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1357::<F>(t10965, t3053, t3082, t3094, t10895, t10952, t1022, t3120, t2250, t360, t1036, t10367);
        let t43267 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1358::<F>(t1032, t10375, t370, t374, t376, t9697, t10908, t3109, t1036, t10446, t1004, t10249, t10413, t10445, t14220, t2979, t3070, t3071, t35, t354, t364, t378, t41649, t43226, t43228, t43233, t43235, t43241, t43246, t6720, t973);
        let (t43273, t43277, t43281, t43285, t43288) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1359::<F>(t10997, t135, t973, t10480, t10483, t248, t3101, t10876, t10877, t10883, t10884, t10473, t361);
    (t43223, t43267, t43273, t43277, t43281, t43285, t43288)
}
