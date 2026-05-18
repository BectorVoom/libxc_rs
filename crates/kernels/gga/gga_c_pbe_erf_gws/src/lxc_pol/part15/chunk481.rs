//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 481/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk481<F: Float>(t143: F, t1533: F, t131: F, t137: F, t142: F, t510: F, t475: F, t522: F, t481: F, t525: F, t169: F, t301: F, t745: F, t784: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2026 = t143 * t1533;
    let t2029 = t131 * t131;
    let t2030 = F::new(1.0) / t2029;
    let t2031 = t2030 * t137;
    let t2032 = t142 * t510;
    let t2033 = t2031 * t2032;
    let t2035 = t475 * t522;
    let t2036 = t142 * t481;
    let t2037 = t525 * t2036;
    let t2042 = t169 * t784 * t745 * t301;
    (t2026, t2029, t2030, t2031, t2032, t2033, t2035, t2036, t2037, t2042)
}
