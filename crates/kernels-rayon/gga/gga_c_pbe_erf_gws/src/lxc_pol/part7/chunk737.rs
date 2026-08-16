//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 737/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk737(t125: f64, t1501: f64, t1533: f64, t1593: f64, t169: f64, t2033: f64, t2036: f64, t2037: f64, t279: f64, t2857: f64, t296: f64, t299: f64, t301: f64, t4867: f64, t523: f64, t526: f64, t5596: f64, t5598: f64, t5601: f64, t5603: f64, t5608: f64, t5612: f64, t5617: f64, t5625: f64, t5629: f64, t5633: f64, t5687: f64, t5690: f64, t5694: f64, t5739: f64, t5881: f64, t5888: f64, t5989: f64, t6023: f64, t6028: f64, t6065: f64) -> f64 {
    let t6067 = t5596 * t279 + 9.0_f64 * t5598 * t2037 + 18.0_f64 * t5601 * t5603 - 0.35922702030763827282e-1_f64 * t5608 - 0.35922702030763827282e-1_f64 * t5612 - t5617 + 2.0_f64 * t1593 * t1501 + 2.0_f64 * t523 * t5625 - t523 * t5629 + t5633 + t5687 - 0.1743404491073215162e-2_f64 * t5690 - t5694 - 2.0_f64 * t1593 * t2033 + t5739 * t296 + 18.0_f64 * t2857 * t2036 * t1533 + t5881 * t526 + 0.20267214298646782767e-1_f64 * t169 * t299 * t4867 * t301 + t523 * t5888 + (t5989 + t6023) * t125 + t6028 + t6065;
    t6067
}
