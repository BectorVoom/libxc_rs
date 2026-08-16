//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta622 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2238;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2239;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2240;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2241;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta622<F: Float>(t12939: F, t13126: F, t2244: F, t2745: F, t868: F, t16693: F, t9682: F, t1409: F, t707: F, t9862: F, t13123: F, t9467: F, t12903: F, t12915: F, t1877: F, t2522: F, t4310: F, t46341: F, t46345: F, t46349: F, t46353: F, t46355: F, t9516: F, t40: F, t4199: F, t9713: F, t41255: F, t41259: F, t41265: F, t1471: F, t31: F, t9898: F, t10913: F, t12606: F, t12950: F, t1430: F, t2250: F, t4007: F, t4010: F, t4104: F, t45872: F, t607: F, t75: F, t767: F, t9258: F, t9288: F, zeta_threshold: F, t52: F, t12961: F, t1431: F, t4012: F, t4015: F, t4111: F, t771: F, t78: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46361, t46362, t46367, t46370, t46371) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2238::<F>(t12939, t13126, t2244, t2745, t868, t16693, t9682, t1409, t707, t9862, t13123, t9467);
        let (t46372, t46373) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2239::<F>(t46371, t12903, t12915, t1877, t2522, t4310, t46341, t46345, t46349, t46353, t46355, t46361, t46362, t46367, t46370, t9516);
        let (t46377, t46384, t46385, t46386, t46389, t46407) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2240::<F>(t40, t4199, t9713, t41255, t41259, t41265, t1471, t31, t9898, t10913, t12606, t12950, t1430, t2244, t2250, t4007, t4010, t4104, t45872, t607, t75, t767, t9258, t9288, zeta_threshold);
        let t46424 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2241::<F>(t52, t10913, t12606, t12961, t1431, t2244, t2250, t4012, t4015, t4111, t45872, t607, t771, t78, t9258, t9288, zeta_threshold);
        let t46426 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2242::<F>(t46407, t46424);
    (t46361, t46367, t46370, t46372, t46373, t46377, t46384, t46385, t46386, t46389, t46426)
}
