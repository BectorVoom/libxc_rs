//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2008/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2008<F: Float>(t23518: F, t6733: F, t23669: F, t995: F, t3158: F, t6796: F, t6802: F, t23600: F, t10336: F, t1920: F, t1949: F, t2966: F, t6805: F) -> (F, F, F, F, F, F, F) {
    let t82683 = t6733 * t23518;
    let t82713 = t23669 * t995;
    let t82716 = t6796 * t3158;
    let t82717 = t82716 * t6802;
    let t82736 = t23600 * t995;
    let t82799 = F::cast_from(0.30461741978670859935e-2_f64) * t1920 * t10336 * t1949;
    let t82809 = t1920 * t2966 * t6805;
    (t82683, t82713, t82716, t82717, t82736, t82799, t82809)
}
