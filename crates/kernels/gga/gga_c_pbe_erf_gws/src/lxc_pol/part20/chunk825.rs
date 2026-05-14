//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 825/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk825<F: Float>(t3379: F, t532: F, t159: F, t285: F, t142: F, t3637: F, t2858: F, t2873: F, t3649: F, t485: F, t395: F, t3652: F, t9779: F, t9781: F, t9784: F, t9789: F, t9794: F, t9796: F, t9799: F, t9802: F) -> (F, F, F, F, F, F) {
    let t10033 = t532 * t3379;
    let t10035 = t10033 * t159 * t285;
    let t10037 = t142 * t3637;
    let t10046 = t2858 * t2873;
    let t10049 = t485 * t3649;
    let t10050 = t10049 * t395;
    let t10051 = 0.97434166666666666667e0 * t10050;
    let t10052 = t485 * t3652;
    let t10053 = t10052 * t395;
    let t10054 = 0.48717083333333333333e0 * t10053;
    let t10063 = 4.0 / 27.0 * t9779 - 4.0 / 9.0 * t9781 - t9784 / 9.0 + t9789 / 3.0 + 4.0 / 27.0 * t9794 + 4.0 / 9.0 * t9796 - t9799 / 9.0 + t9802 / 3.0;
    (t10035, t10037, t10046, t10051, t10054, t10063)
}
