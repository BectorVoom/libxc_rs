//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1603/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1603<F: Float>(t45: F, t57: F, t18272: F, t22671: F, t2375: F, t39825: F, t4377: F, t5825: F, t78: F, t87107: F, t87126: F, t87145: F, t18286: F, t2382: F, t39840: F, t4384: F, t81: F, zeta_threshold: F) -> (F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t87280 = piecewise3::<F>(t151, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39825 * t87145 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t18272 * t5825 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2375 * t87107 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4377 * t22671 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t78 * t87126);
    let t87292 = piecewise3::<F>(t155, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39840 * t87145 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t18286 * t5825 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2382 * t87107 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t4384 * t22671 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t81 * t87126);
    (t87280, t87292)
}
