//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta642 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2350;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2351;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2352;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2353;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta642<F: Float>(t344: F, t42308: F, t60: F, t1597: F, t341: F, t10245: F, t13847: F, t2986: F, t13931: F, t2987: F, t135: F, t13933: F, t973: F, t13532: F, t13784: F, t10213: F, t134: F, t13537: F, t4509: F, t4540: F, t13797: F, t10186: F, t13848: F, t10208: F, t10237: F, t13769: F, t13791: F, t13794: F, t13798: F, t13851: F, t23547: F, t2771: F, t2990: F, t340: F, t343: F, t42799: F, t42830: F, t43071: F, t4510: F, t4531: F, t4532: F, t47679: F, t47697: F, t47742: F, t48120: F, t48169: F, t6733: F, t884: F, t974: F, t13780: F, t13785: F, t13839: F, t42837: F, t10236: F, t12652: F, t10913: F, t13554: F, t13536: F, t12648: F, t13783: F, t4548: F, t698: F, t10235: F, t13770: F, t13840: F, t13852: F, t13855: F, t42842: F, t43028: F, t43038: F) -> (F, F, F, F, F, F, F, F) {
        let (t48180, t48184, t48189, t48191, t48207) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2350::<F>(t344, t42308, t60, t1597, t341, t10245, t13847, t2986, t13931, t2987, t135, t13933, t973);
        let (t48210, t48215, t48217, t48221, t48233) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2351::<F>(t13532, t13784, t2986, t10213, t134, t344, t13537, t4509, t4540, t13797, t1597, t10186, t13848);
        let t48235 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2352::<F>(t10186, t10208, t10237, t10245, t13769, t13791, t13794, t13798, t13851, t23547, t2771, t2986, t2990, t340, t343, t42799, t42830, t43071, t4510, t4531, t4532, t47679, t47697, t47742, t48120, t48169, t48180, t48184, t48189, t48191, t48207, t48210, t48215, t48217, t48221, t48233, t6733, t884, t973, t974);
        let (t48242, t48244, t48250, t48256, t48260, t48265, t48269) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2353::<F>(t10186, t13780, t13785, t13839, t2986, t42837, t10236, t12652, t10913, t13554, t13536, t12648);
        let t48294 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2354::<F>(t13783, t1597, t10237, t2986, t340, t4548, t698, t973, t10186, t10235, t13769, t13770, t13798, t13840, t13852, t13855, t42842, t43028, t43038, t48265, t48269);
    (t48235, t48242, t48244, t48250, t48256, t48260, t48265, t48294)
}
