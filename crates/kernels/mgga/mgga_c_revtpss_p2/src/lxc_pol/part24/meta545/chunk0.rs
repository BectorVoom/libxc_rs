//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1612/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1612<F: Float>(t45: F, t57: F, t18367: F, t22671: F, t2299: F, t4328: F, t5825: F, t766: F, t80: F, t87107: F, t87126: F, t87145: F, t18379: F, t2306: F, t4335: F, t770: F, t83: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t87529 = piecewise3::<F>(t151, F::new(0.0), -F::new(56.0) / F::new(81.0) * t2299 * t87145 + F::new(16.0) / F::new(9.0) * t18367 * t5825 - F::new(2.0) / F::new(3.0) * t80 * t87107 - F::new(8.0) / F::new(9.0) * t4328 * t22671 + F::new(2.0) / F::new(3.0) * t766 * t87126);
    let t87541 = piecewise3::<F>(t155, F::new(0.0), -F::new(56.0) / F::new(81.0) * t2306 * t87145 - F::new(16.0) / F::new(9.0) * t18379 * t5825 - F::new(2.0) / F::new(3.0) * t83 * t87107 - F::new(8.0) / F::new(9.0) * t4335 * t22671 - F::new(2.0) / F::new(3.0) * t770 * t87126);
    (t87529, t87541)
}
