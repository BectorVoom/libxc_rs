//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2350;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2351;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2352;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2353;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta642(t344: f64, t42308: f64, t60: f64, t1597: f64, t341: f64, t10245: f64, t13847: f64, t2986: f64, t13931: f64, t2987: f64, t135: f64, t13933: f64, t973: f64, t13532: f64, t13784: f64, t10213: f64, t134: f64, t13537: f64, t4509: f64, t4540: f64, t13797: f64, t10186: f64, t13848: f64, t10208: f64, t10237: f64, t13769: f64, t13791: f64, t13794: f64, t13798: f64, t13851: f64, t23547: f64, t2771: f64, t2990: f64, t340: f64, t343: f64, t42799: f64, t42830: f64, t43071: f64, t4510: f64, t4531: f64, t4532: f64, t47679: f64, t47697: f64, t47742: f64, t48120: f64, t48169: f64, t6733: f64, t884: f64, t974: f64, t13780: f64, t13785: f64, t13839: f64, t42837: f64, t10236: f64, t12652: f64, t10913: f64, t13554: f64, t13536: f64, t12648: f64, t13783: f64, t4548: f64, t698: f64, t10235: f64, t13770: f64, t13840: f64, t13852: f64, t13855: f64, t42842: f64, t43028: f64, t43038: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48180, t48184, t48189, t48191, t48207) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2350(t344, t42308, t60, t1597, t341, t10245, t13847, t2986, t13931, t2987, t135, t13933, t973);
        let (t48210, t48215, t48217, t48221, t48233) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2351(t13532, t13784, t2986, t10213, t134, t344, t13537, t4509, t4540, t13797, t1597, t10186, t13848);
        let t48235 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2352(t10186, t10208, t10237, t10245, t13769, t13791, t13794, t13798, t13851, t23547, t2771, t2986, t2990, t340, t343, t42799, t42830, t43071, t4510, t4531, t4532, t47679, t47697, t47742, t48120, t48169, t48180, t48184, t48189, t48191, t48207, t48210, t48215, t48217, t48221, t48233, t6733, t884, t973, t974);
        let (t48242, t48244, t48250, t48256, t48260, t48265, t48269) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2353(t10186, t13780, t13785, t13839, t2986, t42837, t10236, t12652, t10913, t13554, t13536, t12648);
        let t48294 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2354(t13783, t1597, t10237, t2986, t340, t4548, t698, t973, t10186, t10235, t13769, t13770, t13798, t13840, t13852, t13855, t42842, t43028, t43038, t48265, t48269);
    (t48235, t48242, t48244, t48250, t48256, t48260, t48265, t48294)
}
