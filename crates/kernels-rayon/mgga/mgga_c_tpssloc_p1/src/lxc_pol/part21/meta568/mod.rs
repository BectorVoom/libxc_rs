//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2276;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2277;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2278;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta568(t1246: f64, t19189: f64, t19120: f64, t493: f64, t1243: f64, t19045: f64, t3612: f64, t5011: f64, t1755: f64, t11881: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1758: f64, t18572: f64, t19166: f64, t19170: f64, t19174: f64, t19176: f64, t19180: f64, t3604: f64, t3610: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5073: f64, t5076: f64, t5086: f64, t6168: f64, t6257: f64, t6265: f64, t19164: f64, t1241: f64, t1235: f64, t6150: f64, t1760: f64, t5088: f64, t3598: f64, t1251: f64, t6267: f64, t6243: f64, t11606: f64, t1238: f64, t15820: f64, t1761: f64, t18287: f64, t19121: f64, t3487: f64, t3593: f64, t4945: f64, t498: f64, t5055: f64, t5060: f64, t6268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19190, t19197, t19201, t19203, t19204, t19207) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2276(t1246, t19189, t19120, t493, t1243, t19045, t3612, t5011, t1755, t11881, t1201, t1244, t1247, t1249, t1729, t1758, t18572, t19166, t19170, t19174, t19176, t19180, t3604, t3610, t470, t494, t4964, t5064, t5073, t5076, t5086, t6168, t6257, t6265);
        let (t19208, t19209, t19211, t19214, t19220, t19226) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2277(t19164, t19207, t1241, t1235, t6150, t1760, t5088, t3598, t1251, t6267, t6243, t11606);
        let t19231 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2278(t1238, t15820, t1761, t18287, t19121, t19209, t19211, t19214, t19220, t19226, t3487, t3593, t4945, t498, t5055, t5060, t6268);
    (t19190, t19197, t19201, t19203, t19204, t19208, t19209, t19211, t19214, t19220, t19226, t19231)
}
