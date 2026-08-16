//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2276;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2277;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta568<F: Float>(t1246: F, t19189: F, t19120: F, t493: F, t1243: F, t19045: F, t3612: F, t5011: F, t1755: F, t11881: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1758: F, t18572: F, t19166: F, t19170: F, t19174: F, t19176: F, t19180: F, t3604: F, t3610: F, t470: F, t494: F, t4964: F, t5064: F, t5073: F, t5076: F, t5086: F, t6168: F, t6257: F, t6265: F, t19164: F, t1241: F, t1235: F, t6150: F, t1760: F, t5088: F, t3598: F, t1251: F, t6267: F, t6243: F, t11606: F, t1238: F, t15820: F, t1761: F, t18287: F, t19121: F, t3487: F, t3593: F, t4945: F, t498: F, t5055: F, t5060: F, t6268: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19190, t19197, t19201, t19203, t19204, t19207) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2276::<F>(t1246, t19189, t19120, t493, t1243, t19045, t3612, t5011, t1755, t11881, t1201, t1244, t1247, t1249, t1729, t1758, t18572, t19166, t19170, t19174, t19176, t19180, t3604, t3610, t470, t494, t4964, t5064, t5073, t5076, t5086, t6168, t6257, t6265);
        let (t19208, t19209, t19211, t19214, t19220, t19226) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2277::<F>(t19164, t19207, t1241, t1235, t6150, t1760, t5088, t3598, t1251, t6267, t6243, t11606);
        let t19231 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2278::<F>(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
    (t19190, t19197, t19201, t19203, t19204, t19208, t19209, t19211, t19214, t19220, t19226, t19231)
}
