//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 995/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk995<F: Float>(t57: F, t202: F, t2382: F, t606: F, t10326: F, t10356: F, t2258: F, t81: F, t10455: F, t150: F, t190: F, t80: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t155 = t57 <= zeta_threshold;
    let t10457 = F::new(1.0) / t202 / t57;
    let t10460 = t2382 * t606;
    let t10466 = piecewise3::<F>(t155, F::new(0.0), F::new(8.0) / F::new(27.0) * t10457 * t10356 + F::new(4.0) / F::new(3.0) * t10460 * t2258 - F::new(4.0) / F::new(3.0) * t81 * t10326);
    let t10467 = t10455 + t10466;
    let t10468 = t150 * t10467;
    let t10469 = t10468 * t190;
    let t10472 = t80 * t606;
    (t10457, t10460, t10467, t10468, t10469, t10472)
}
