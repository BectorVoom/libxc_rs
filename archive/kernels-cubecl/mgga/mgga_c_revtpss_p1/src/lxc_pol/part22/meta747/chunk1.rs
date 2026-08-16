//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2820/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2820<F: Float>(t273: F, t270: F, t276: F, t39484: F, t9303: F, t931: F, t2922: F, t275: F, t2925: F, t41306: F, t2866: F, t2923: F) -> (F, F, F, F, F, F, F, F) {
    let t41382 = F::powf(t273, -F::cast_from(0.25e1_f64));
    let t41401 = F::cast_from(1.0_f64) / t276 / t39484 / t270 / F::cast_from(96.0_f64);
    let t41441 = t9303 * t931;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    let t41501 = t2925 * t2925;
    let t41502 = F::cast_from(1.0_f64) / t41501;
    let t41520 = F::cast_from(0.96141975308641975307e-1_f64) * t41306;
    let t41549 = F::cast_from(0.18467901234567901234e0_f64) * t41306;
    let t41578 = t2866 * t2923;
    (t41382, t41401, t41441, t41499, t41502, t41520, t41549, t41578)
}
