//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1244/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1244<F: Float>(t25866: F, t7235: F, t2470: F, t26049: F, t7284: F, t2453: F, t555: F, t25898: F, t136: F, t137: F, t2022: F, t1399: F, t2438: F) -> (F, F, F, F, F, F, F) {
    let t94376 = F::cast_from(18.0_f64) * t7235 * t25866;
    let t94377 = t26049 * t2470;
    let t94378 = t7284 * t94377;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94385 = t2022 * t136 * t137;
    let t94386 = t2438 * t1399;
    (t94376, t94377, t94378, t94382, t94383, t94385, t94386)
}
