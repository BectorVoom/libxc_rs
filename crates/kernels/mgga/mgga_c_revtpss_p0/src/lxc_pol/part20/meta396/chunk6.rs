//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1463/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1463<F: Float>(t2944: F, t41245: F, t41250: F, t41255: F, t41260: F, t41265: F, t41267: F, t41273: F, t41275: F, t41279: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F) -> (F, F) {
    let t41668 = t2944 * t2944;
    let t41672 = F::cast_from(0.16979925925925925926e1_f64) * t41245;
    let t41686 = t41672 - F::cast_from(0.27785333333333333334e0_f64) * t41250 + F::cast_from(0.83356e0_f64) * t41255 - F::cast_from(0.13892666666666666667e0_f64) * t41260 + F::cast_from(0.125034e1_f64) * t41265 - F::cast_from(0.166712e1_f64) * t41267 + F::cast_from(0.55570666666666666666e0_f64) * t41273 + F::cast_from(0.166712e1_f64) * t41275 - F::cast_from(0.125034e1_f64) * t41279 + F::cast_from(0.13892666666666666667e1_f64) * t41281 - F::cast_from(0.55570666666666666668e0_f64) * t41283 - F::cast_from(0.69463333333333333334e0_f64) * t41285 - F::cast_from(0.23154444444444444445e0_f64) * t41287 + F::cast_from(0.27785333333333333333e0_f64) * t41289;
    (t41668, t41686)
}
