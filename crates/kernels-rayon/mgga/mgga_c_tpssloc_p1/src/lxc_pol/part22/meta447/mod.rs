//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1801;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1802;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta447(t1343: f64, t19732: f64, t820: f64, t120: f64, t6387: f64, t5248: f64, t5250: f64, t5234: f64, t5245: f64, t12283: f64, t6396: f64, t3805: f64, t3807: f64, t16306: f64, t6394: f64, t16305: f64, t16225: f64, t16311: f64, t1825: f64, t5308: f64, t16224: f64, t12286: f64, t1341: f64, t16239: f64, t16241: f64, t16269: f64, t16290: f64, t16294: f64, t16317: f64, t16325: f64, t16331: f64, t16338: f64, t16341: f64, t3778: f64, t3803: f64, t5246: f64, t5252: f64, t6390: f64, t6417: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19868, t19871) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1801(t1343, t19732, t820, t120, t6387);
        let (t19873, t19876, t19879, t19882, t19886) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1802(t19871, t5248, t5250, t5234, t5245, t12283, t6396, t3805, t3807, t16306, t6394, t16305);
        let (t19890, t19894, t19899) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1803(t16225, t16311, t16305, t1825, t5308, t16224, t12286, t1341, t16239, t16241, t16269, t16290, t16294, t16317, t16325, t16331, t16338, t16341, t19868, t19873, t19876, t19879, t19882, t19886, t3778, t3803, t5246, t5252, t6390, t6417);
    (t19868, t19871, t19873, t19876, t19879, t19882, t19886, t19890, t19894, t19899)
}
