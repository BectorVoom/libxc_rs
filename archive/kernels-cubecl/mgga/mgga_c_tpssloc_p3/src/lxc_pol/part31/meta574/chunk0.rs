//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1809/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1809<F: Float>(t25055: F, t81591: F, t25217: F, t6547: F, t25060: F, t82209: F, t82211: F, t25192: F, t81651: F, t82074: F, t82259: F, t25054: F) -> (F, F, F, F, F, F, F, F) {
    let t87786 = t81591 * t25055;
    let t87796 = t6547 * t25217;
    let t87804 = t6547 * t25060;
    let t87806 = F::cast_from(0.25587863262083522346e0_f64) * t82209;
    let t87807 = F::cast_from(0.12793931631041761173e0_f64) * t82211;
    let t87835 = t81651 * t82074 * t25192;
    let t87847 = F::cast_from(0.12793931631041761173e0_f64) * t82259;
    let t87873 = t81651 * t82074 * t25054;
    (t87786, t87796, t87804, t87806, t87807, t87835, t87847, t87873)
}
