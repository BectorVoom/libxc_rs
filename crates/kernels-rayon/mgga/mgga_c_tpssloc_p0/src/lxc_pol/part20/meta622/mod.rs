//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2238;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2239;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2240;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2241;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta622(t12939: f64, t13126: f64, t2244: f64, t2745: f64, t868: f64, t16693: f64, t9682: f64, t1409: f64, t707: f64, t9862: f64, t13123: f64, t9467: f64, t12903: f64, t12915: f64, t1877: f64, t2522: f64, t4310: f64, t46341: f64, t46345: f64, t46349: f64, t46353: f64, t46355: f64, t9516: f64, t40: f64, t4199: f64, t9713: f64, t41255: f64, t41259: f64, t41265: f64, t1471: f64, t31: f64, t9898: f64, t10913: f64, t12606: f64, t12950: f64, t1430: f64, t2250: f64, t4007: f64, t4010: f64, t4104: f64, t45872: f64, t607: f64, t75: f64, t767: f64, t9258: f64, t9288: f64, zeta_threshold: f64, t52: f64, t12961: f64, t1431: f64, t4012: f64, t4015: f64, t4111: f64, t771: f64, t78: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46361, t46362, t46367, t46370, t46371) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2238(t12939, t13126, t2244, t2745, t868, t16693, t9682, t1409, t707, t9862, t13123, t9467);
        let (t46372, t46373) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2239(t46371, t12903, t12915, t1877, t2522, t4310, t46341, t46345, t46349, t46353, t46355, t46361, t46362, t46367, t46370, t9516);
        let (t46377, t46384, t46385, t46386, t46389, t46407) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2240(t40, t4199, t9713, t41255, t41259, t41265, t1471, t31, t9898, t10913, t12606, t12950, t1430, t2244, t2250, t4007, t4010, t4104, t45872, t607, t75, t767, t9258, t9288, zeta_threshold);
        let t46424 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2241(t52, t10913, t12606, t12961, t1431, t2244, t2250, t4012, t4015, t4111, t45872, t607, t771, t78, t9258, t9288, zeta_threshold);
        let t46426 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2242(t46407, t46424);
    (t46361, t46367, t46370, t46372, t46373, t46377, t46384, t46385, t46386, t46389, t46426)
}
