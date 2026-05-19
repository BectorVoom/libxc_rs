//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 737/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk737<F: Float>(t125: F, t1501: F, t1533: F, t1593: F, t169: F, t2033: F, t2036: F, t2037: F, t279: F, t2857: F, t296: F, t299: F, t301: F, t4867: F, t523: F, t526: F, t5596: F, t5598: F, t5601: F, t5603: F, t5608: F, t5612: F, t5617: F, t5625: F, t5629: F, t5633: F, t5687: F, t5690: F, t5694: F, t5739: F, t5881: F, t5888: F, t5989: F, t6023: F, t6028: F, t6065: F) -> F {
    let t6067 = t5596 * t279 + F::new(9.0) * t5598 * t2037 + F::new(18.0) * t5601 * t5603 - F::cast_from(0.35922702030763827282e-1_f64) * t5608 - F::cast_from(0.35922702030763827282e-1_f64) * t5612 - t5617 + F::new(2.0) * t1593 * t1501 + F::new(2.0) * t523 * t5625 - t523 * t5629 + t5633 + t5687 - F::cast_from(0.1743404491073215162e-2_f64) * t5690 - t5694 - F::new(2.0) * t1593 * t2033 + t5739 * t296 + F::new(18.0) * t2857 * t2036 * t1533 + t5881 * t526 + F::cast_from(0.20267214298646782767e-1_f64) * t169 * t299 * t4867 * t301 + t523 * t5888 + (t5989 + t6023) * t125 + t6028 + t6065;
    t6067
}
