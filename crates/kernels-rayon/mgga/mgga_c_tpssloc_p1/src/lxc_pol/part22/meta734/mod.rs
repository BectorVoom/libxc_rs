//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta734 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2410;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2411;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta734(t59759: f64, t59761: f64, t60308: f64, t60310: f64, t60312: f64, t68638: f64, t68640: f64, t68643: f64, t68646: f64, t68649: f64, t68695: f64, t68697: f64, t68785: f64, t68798: f64, t68812: f64, t68825: f64, t68839: f64, t68851: f64, t68864: f64, t893: f64, t913: f64, t21303: f64, t42023: f64, t14473: f64, t5808: f64, t5790: f64, t950: f64, t4475: f64, t49532: f64, t4472: f64, t5811: f64, t959: f64, t1589: f64, t60848: f64, t68767: f64, t68769: f64, t68771: f64, t68773: f64, t68775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68877 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2410(t59759, t59761, t60308, t60310, t60312, t68638, t68640, t68643, t68646, t68649, t68695, t68697);
        let (t68883, t68885) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2411(t68785, t68798, t68812, t68825, t68839, t68851, t68864, t68877, t893, t913, t21303, t42023);
        let (t68887, t68888, t68891, t68894, t68896, t68897) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2412(t14473, t5808, t5790, t950, t4475, t49532, t4472, t5811, t959, t1589, t60848, t68767, t68769, t68771, t68773, t68775, t68883, t68885);
    (t68883, t68885, t68887, t68888, t68891, t68894, t68896, t68897)
}
