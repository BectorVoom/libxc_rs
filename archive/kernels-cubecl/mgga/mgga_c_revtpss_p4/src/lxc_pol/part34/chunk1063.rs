//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1063/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1063<F: Float>(t476: F, t52: F, t475: F, t467: F, t1785: F, t6594: F, t12678: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F) -> (F, F, F) {
    let t24677 = t476 * t476;
    let t24679 = F::cast_from(1.0_f64) / t52 / t24677;
    let t24680 = t475 * t24679;
    let t24681 = t467 * t24680;
    let t24684 = t1785 * t6594;
    let t24697 = -t12678 + F::cast_from(0.11111111111111111111e-1_f64) * t16706 + F::cast_from(0.55555555555555555555e-2_f64) * t20283 - F::cast_from(0.16666666666666666667e-1_f64) * t20285 - F::cast_from(0.83333333333333333334e-2_f64) * t20287 + F::cast_from(0.92592592592592592592e-2_f64) * t24230 - F::cast_from(0.33333333333333333333e-1_f64) * t24234 - F::cast_from(0.16666666666666666666e-1_f64) * t24238 + F::cast_from(0.50000000000000000001e-1_f64) * t24242 + F::cast_from(0.50000000000000000001e-1_f64) * t24246 + F::cast_from(0.83333333333333333333e-2_f64) * t24250;
    (t24681, t24684, t24697)
}
