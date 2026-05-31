//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1233/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1233<F: Float>(t122647: F, t28067: F, t8713: F, t9593: F, t28196: F, t28198: F, t28187: F, t8698: F, t32662: F, t7898: F, t28167: F, t38099: F, t5627: F) -> (F, F, F, F, F) {
    let t128266 = F::cast_from(3.0_f64) * t122647 * t28067;
    let t128267 = t8713 * t9593;
    let t128270 = F::cast_from(2.0_f64) * t28196 * t128267 * t28198;
    let t128273 = t8698 * t28187;
    let t128274 = t7898 * t32662;
    let t128277 = F::cast_from(6.0_f64) * t28167 * t38099 * t5627;
    (t128266, t128270, t128273, t128274, t128277)
}
