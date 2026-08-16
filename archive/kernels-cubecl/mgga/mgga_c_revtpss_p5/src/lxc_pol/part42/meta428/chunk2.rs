//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1493/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1493<F: Float>(t116: F, t31653: F, t31027: F, t31629: F, t31636: F, t31032: F, t31643: F, t117918: F, t117920: F, t117927: F, t117936: F, t117938: F, t117940: F, t117997: F, t1513: F, t2357: F, t31439: F, t31443: F, t36308: F, t36315: F) -> (F, F) {
    let t118630 = t116 * t31653;
    let t118649 = t31027 * t31629;
    let t118651 = t31027 * t31636;
    let t118653 = t31032 * t31643;
    let t118655 = -F::cast_from(5.0_f64) / F::cast_from(2.0_f64) * t36308 * t117997 * t31439 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t36315 * t2357 * t1513 * t31443 + t117918 - t117920 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t117927 - F::cast_from(110.0_f64) / F::cast_from(27.0_f64) * t117936 + F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t117938 + F::cast_from(110.0_f64) / F::cast_from(27.0_f64) * t117940 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t118649 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t118651 - F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t118653;
    (t118630, t118655)
}
