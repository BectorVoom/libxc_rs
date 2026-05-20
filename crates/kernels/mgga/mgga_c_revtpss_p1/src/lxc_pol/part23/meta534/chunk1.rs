//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2065/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2065<F: Float>(t21720: F, t21808: F, t10301: F, t10309: F, t13269: F, t13272: F, t1497: F, t21661: F, t21663: F, t21674: F, t21677: F, t21682: F, t2242: F, t2247: F, t4173: F, t4178: F, t4241: F, t5816: F, t5872: F, t603: F, t644: F, t91: F) -> (F, F) {
    let t21809 = t21720 + t21808;
    let t21812 = F::new(20.0) * t10301 * t5816 - F::new(120.0) * t10309 * t21674 - F::new(8.0) * t13269 * t1497 + F::new(40.0) * t13272 * t4178 + t21661 * t91 - F::new(4.0) * t21663 * t644 + F::new(40.0) * t21677 * t2247 + F::new(20.0) * t21682 * t2247 - F::new(4.0) * t21809 * t603 - F::new(4.0) * t2242 * t5872 - F::new(8.0) * t4173 * t4241;
    (t21809, t21812)
}
