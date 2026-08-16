//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2656;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta677(t473: f64, t6695: f64, t1214: f64, t3759: f64, t6587: f64, t1280: f64, t21082: f64, t21471: f64, t5284: f64, t5332: f64, t1269: f64, t1287: f64, t6622: f64, t6573: f64, t1234: f64, t12756: f64, t1285: f64, t1291: f64, t12966: f64, t12987: f64, t1770: f64, t1825: f64, t21333: f64, t21518: f64, t21521: f64, t21524: f64, t21527: f64, t21535: f64, t21538: f64, t3670: f64, t460: f64, t490: f64, t5216: f64, t5478: f64, t5494: f64, t6564: f64, t6714: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21541, t21542, t21551, t21554, t21558, t21562) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2656(t473, t6695, t1214, t3759, t6587, t1280, t21082, t21471, t5284, t5332, t1269, t1287, t6622);
        let (t21565, t21568) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2657(t3759, t6573, t1234, t12756, t1285, t1291, t12966, t12987, t1770, t1825, t21333, t21518, t21521, t21524, t21527, t21535, t21538, t21542, t21551, t21554, t21558, t21562, t3670, t460, t490, t5216, t5478, t5494, t6564, t6714);
    (t21541, t21542, t21551, t21554, t21558, t21562, t21565, t21568)
}
