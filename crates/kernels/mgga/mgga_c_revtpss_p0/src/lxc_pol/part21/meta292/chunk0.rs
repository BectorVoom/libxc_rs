//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1535/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1535<F: Float>(t45: F, t10439: F, t10440: F, t2401: F, t750: F, t200: F, t2375: F, t606: F, t10326: F, t10356: F, t2258: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t10442 = F::new(24.0) * t10439 * t10440;
    let t10443 = t2401 * t750;
    let t10444 = F::new(3.0) * t10443;
    let t10446 = F::new(1.0) / t200 / t45;
    let t10449 = t2375 * t606;
    let t10455 = piecewise3::<F>(t151, F::new(0.0), -F::new(8.0) / F::new(27.0) * t10446 * t10356 + F::new(4.0) / F::new(3.0) * t10449 * t2258 + F::new(4.0) / F::new(3.0) * t78 * t10326);
    (t10442, t10443, t10444, t10446, t10455)
}
