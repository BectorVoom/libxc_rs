//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 114/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk114<F: Float>(t225: F, t378: F, t359: F, t342: F) -> (F, F, F, F, F) {
    let t379 = t378 * t225;
    let t380 = t225 * t359;
    let t381 = t380 * t378;
    let t384 = F::new(1.0) + F::cast_from(0.65854491829355115987e0_f64) * t342 * t381;
    let t385 = F::new(1.0) / t384;
    (t379, t380, t381, t384, t385)
}
