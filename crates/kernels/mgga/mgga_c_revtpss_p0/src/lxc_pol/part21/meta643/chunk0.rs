//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2428/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2428<F: Float>(t2925: F, t41306: F, t11545: F, t914: F, t2866: F, t2923: F, t11384: F, t910: F, t275: F, t2872: F, t2922: F, t41245: F) -> (F, F, F, F, F, F, F, F) {
    let t41501 = t2925 * t2925;
    let t41502 = F::cast_from(1.0_f64) / t41501;
    let t41520 = F::cast_from(0.96141975308641975307e-1_f64) * t41306;
    let t41549 = F::cast_from(0.18467901234567901234e0_f64) * t41306;
    let t41571 = t11545 * t914;
    let t41578 = t2866 * t2923;
    let t41583 = t910 * t11384;
    let t41588 = t275 / t2922 / t2872;
    let t41592 = F::cast_from(0.13388493827160493828e1_f64) * t41245;
    (t41502, t41520, t41549, t41571, t41578, t41583, t41588, t41592)
}
