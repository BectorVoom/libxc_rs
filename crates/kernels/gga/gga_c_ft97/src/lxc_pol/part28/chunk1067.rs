//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1067/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1067<F: Float>(t3238: F, t32457: F, t103163: F, t1332: F, t108: F, t1286: F, t1337: F, t137525: F, t145585: F, t25601: F, t25847: F, t25863: F, t28: F, t32016: F, t32378: F, t32387: F, t32392: F, t34581: F, t34784: F, t369: F, t438: F, t5495: F, t5501: F, t5748: F, t6414: F, t6455: F, t984: F) -> (F, F, F) {
    let t145741 = t3238 * t32457;
    let t145761 = t103163 * t1332;
    let t145769 = t5501 * t137525 * t25601 / F::cast_from(9.0_f64) - t32016 * t25863 / F::cast_from(18.0_f64) - F::cast_from(2.0_f64) * t145741 + t1286 * t28 * t25847 * t1337 / F::cast_from(3.0_f64) + t1286 * t28 * t369 * t145585 * t108 / F::cast_from(6.0_f64) + t1286 * t28 * t32378 * t984 / F::cast_from(6.0_f64) - t438 * t34784 + t5495 * t34581 / F::cast_from(6.0_f64) - t6414 * t32392 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) * t145761 + t1286 * t28 * t6455 * t5748 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6414 * t32387;
    (t145741, t145761, t145769)
}
