//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2040/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2040<F: Float>(t1887: F, t81959: F, t22690: F, t23171: F, t25319: F, t23143: F, t7525: F, t25238: F, t6579: F, t22893: F, t23164: F, t25312: F) -> (F, F, F, F, F) {
    let t87642 = t81959 * t1887;
    let t87653 = t23171 * t22690 * t25319;
    let t87666 = t23143 * t7525;
    let t87668 = t6579 * t25238;
    let t87669 = F::cast_from(0.38381794893125283518e-1_f64) * t87668;
    let t87679 = t23164 * t22893 * t25312;
    (t87642, t87653, t87666, t87669, t87679)
}
