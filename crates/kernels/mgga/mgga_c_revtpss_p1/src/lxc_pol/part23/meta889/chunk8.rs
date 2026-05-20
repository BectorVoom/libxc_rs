//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2827/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2827<F: Float>(t57: F, t14458: F, t1491: F, t18281: F, t18379: F, t19680: F, t22671: F, t22688: F, t2306: F, t4186: F, t4335: F, t5825: F, t606: F, t76397: F, t770: F, t83: F, zeta_threshold: F) -> F {
    let t155 = t57 <= zeta_threshold;
    let t76419 = piecewise3::<F>(t155, F::new(0.0), -F::new(56.0) / F::new(81.0) * t2306 * t22688 * t606 - F::new(8.0) / F::new(9.0) * t18379 * t4186 - F::new(8.0) / F::new(9.0) * t1491 * t19680 - F::new(2.0) / F::new(3.0) * t14458 * t5825 - F::new(2.0) / F::new(3.0) * t4335 * t18281 - F::new(2.0) / F::new(9.0) * t83 * t22671 * t606 - F::new(2.0) / F::new(3.0) * t770 * t76397);
    t76419
}
