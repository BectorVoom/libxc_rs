//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta734 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2410;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2411;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta734<F: Float>(t59759: F, t59761: F, t60308: F, t60310: F, t60312: F, t68638: F, t68640: F, t68643: F, t68646: F, t68649: F, t68695: F, t68697: F, t68785: F, t68798: F, t68812: F, t68825: F, t68839: F, t68851: F, t68864: F, t893: F, t913: F, t21303: F, t42023: F, t14473: F, t5808: F, t5790: F, t950: F, t4475: F, t49532: F, t4472: F, t5811: F, t959: F, t1589: F, t60848: F, t68767: F, t68769: F, t68771: F, t68773: F, t68775: F) -> (F, F, F, F, F, F, F, F) {
        let t68877 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2410::<F>(t59759, t59761, t60308, t60310, t60312, t68638, t68640, t68643, t68646, t68649, t68695, t68697);
        let (t68883, t68885) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2411::<F>(t68785, t68798, t68812, t68825, t68839, t68851, t68864, t68877, t893, t913, t21303, t42023);
        let (t68887, t68888, t68891, t68894, t68896, t68897) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2412::<F>(t14473, t5808, t5790, t950, t4475, t49532, t4472, t5811, t959, t1589, t60848, t68767, t68769, t68771, t68773, t68775, t68883, t68885);
    (t68883, t68885, t68887, t68888, t68891, t68894, t68896, t68897)
}
