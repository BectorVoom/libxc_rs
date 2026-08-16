//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1793/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1793<F: Float>(t81615: F, t7524: F, t81612: F, t81613: F, t4250: F, t81749: F, t23145: F, t4166: F, t22690: F, t234: F, t7496: F, t776: F, t81792: F) -> (F, F, F, F, F, F) {
    let t87167 = F::cast_from(0.16449340668482264365e-1_f64) * t81615;
    let t87177 = t81612 * t81613 * t7524;
    let t87197 = t81749 * t4250;
    let t87199 = t4166 * t23145;
    let t87202 = t22690 * t234;
    let t87205 = t81792 * t87202 * t7496 * t776;
    (t87167, t87177, t87197, t87199, t87202, t87205)
}
