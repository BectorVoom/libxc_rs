//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1353/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1353<F: Float>(t2925: F, t41306: F, t275: F, t2872: F, t2922: F, t41245: F, t315: F, t41235: F, t2941: F, t2966: F, t302: F, t2969: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t41501 = t2925 * t2925;
    let t41502 = F::cast_from(1.0_f64) / t41501;
    let t41520 = F::cast_from(0.96141975308641975307e-1_f64) * t41306;
    let t41549 = F::cast_from(0.18467901234567901234e0_f64) * t41306;
    let t41588 = t275 / t2922 / t2872;
    let t41592 = F::cast_from(0.13388493827160493828e1_f64) * t41245;
    let t41610 = F::cast_from(0.31003950617283950618e1_f64) * t41306;
    let t41658 = t315 * t41235;
    let t41667 = t302 / t2966 / t2941;
    let t41672 = F::cast_from(0.16979925925925925926e1_f64) * t41245;
    let t41690 = F::cast_from(0.5356037037037037037e1_f64) * t41306;
    let t41738 = t2966 * t2966;
    let t41740 = t302 / t41738;
    let t41741 = t2969 * t2969;
    (t41502, t41520, t41549, t41588, t41592, t41610, t41658, t41667, t41672, t41690, t41740, t41741)
}
