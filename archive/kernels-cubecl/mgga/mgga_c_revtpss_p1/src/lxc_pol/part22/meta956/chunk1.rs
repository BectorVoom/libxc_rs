//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3201/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3201<F: Float>(t4241: F, t21661: F, t602: F, t2246: F, t5812: F, t10309: F, t13269: F, t13272: F, t13286: F, t13289: F, t13420: F, t1497: F, t21663: F, t21809: F, t2242: F, t2247: F, t2248: F, t2315: F, t4173: F, t4178: F, t5872: F, t60221: F, t60248: F, t644: F) -> F {
    let t60667 = t4241 * t4241;
    let t60670 = t21661 * t602;
    let t60673 = t5812 * t2246;
    let t60692 = -F::cast_from(120.0_f64) * t10309 * t2248 * t5872 + F::cast_from(40.0_f64) * t21809 * t2247 * t644 - F::cast_from(16.0_f64) * t13269 * t4241 + F::cast_from(80.0_f64) * t13272 * t13286 + F::cast_from(40.0_f64) * t13272 * t13289 - F::cast_from(8.0_f64) * t13420 * t4173 - F::cast_from(8.0_f64) * t1497 * t60248 - F::cast_from(4.0_f64) * t21663 * t2315 - F::cast_from(8.0_f64) * t21809 * t2242 + F::cast_from(40.0_f64) * t2247 * t60667 + F::cast_from(20.0_f64) * t2248 * t60673 + F::cast_from(80.0_f64) * t4178 * t60221 - F::cast_from(8.0_f64) * t60670 * t644;
    t60692
}
