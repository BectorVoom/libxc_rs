//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3480/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3480<F: Float>(t1042: F, t11714: F, t1592: F, t3127: F, t42665: F, t42672: F, t4825: F, t53290: F, t53293: F, t53926: F, t54419: F, t6308: F, t6312: F, t6331: F, t65444: F, t65446: F, t65454: F, t65456: F, t65459: F, t65462: F) -> F {
    let t65468 = -F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t1042 * t54419 * t1592 + F::cast_from(0.3811023832717309953e-3_f64) * t65444 - F::cast_from(0.3811023832717309953e-3_f64) * t65446 + F::cast_from(0.42874018118069736972e-3_f64) * t42665 * t6308 - F::cast_from(0.21437009059034868486e-3_f64) * t42672 * t6312 + F::cast_from(0.30488190661738479624e-2_f64) * t11714 * t6331 - F::cast_from(0.3811023832717309953e-3_f64) * t65454 - F::cast_from(0.30488190661738479624e-2_f64) * t65456 - F::cast_from(0.19055119163586549766e-2_f64) * t65459 + F::cast_from(0.63517063878621832552e-3_f64) * t65462 + F::cast_from(0.30488190661738479624e-2_f64) * t53926 * t4825 - F::cast_from(0.30488190661738479624e-2_f64) * t53290 - F::cast_from(0.19055119163586549765e-3_f64) * t53293;
    t65468
}
