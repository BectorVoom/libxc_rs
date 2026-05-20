//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1104/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1104<F: Float>(t11545: F, t291: F, t2942: F, t941: F, t11410: F, t954: F, t2986: F, t960: F, t11467: F, t973: F, t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F) -> (F, F, F, F, F, F) {
    let t11547 = F::new(0.621814e-1) * t11545 * t291;
    let t11548 = t941 * t2942;
    let t11551 = t11410 * t954;
    let t11554 = t960 * t2986;
    let t11557 = t11467 * t973;
    let t11560 = F::cast_from(0.28842592592592592592e-1_f64) * t11132;
    let t11571 = -t11560 - F::cast_from(0.12361111111111111111e-1_f64) * t11134 + F::cast_from(0.61805555555555555556e-2_f64) * t11136 - F::cast_from(0.18541666666666666667e-1_f64) * t11138 + F::cast_from(0.92708333333333333334e-2_f64) * t11140 - F::cast_from(0.10300925925925925926e-1_f64) * t11147 + F::cast_from(0.37083333333333333333e-1_f64) * t11153 - F::cast_from(0.18541666666666666666e-1_f64) * t11158 - F::cast_from(0.55625000000000000001e-1_f64) * t11162 + F::cast_from(0.55625000000000000001e-1_f64) * t11167 - F::cast_from(0.92708333333333333333e-2_f64) * t11171;
    (t11547, t11548, t11551, t11554, t11557, t11571)
}
