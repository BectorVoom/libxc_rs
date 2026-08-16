//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2656;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta677<F: Float>(t473: F, t6695: F, t1214: F, t3759: F, t6587: F, t1280: F, t21082: F, t21471: F, t5284: F, t5332: F, t1269: F, t1287: F, t6622: F, t6573: F, t1234: F, t12756: F, t1285: F, t1291: F, t12966: F, t12987: F, t1770: F, t1825: F, t21333: F, t21518: F, t21521: F, t21524: F, t21527: F, t21535: F, t21538: F, t3670: F, t460: F, t490: F, t5216: F, t5478: F, t5494: F, t6564: F, t6714: F) -> (F, F, F, F, F, F, F, F) {
        let (t21541, t21542, t21551, t21554, t21558, t21562) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2656::<F>(t473, t6695, t1214, t3759, t6587, t1280, t21082, t21471, t5284, t5332, t1269, t1287, t6622);
        let (t21565, t21568) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2657::<F>(t3759, t6573, t1234, t12756, t1285, t1291, t12966, t12987, t1770, t1825, t21333, t21518, t21521, t21524, t21527, t21535, t21538, t21542, t21551, t21554, t21558, t21562, t3670, t460, t490, t5216, t5478, t5494, t6564, t6714);
    (t21541, t21542, t21551, t21554, t21558, t21562, t21565, t21568)
}
