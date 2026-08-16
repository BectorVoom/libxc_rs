//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1142/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1142<F: Float>(t652: F, t6535: F, t1976: F, t671: F, t25: F, t776: F, t154: F, t781: F) -> (F, F, F, F) {
    let t6537 = F::cast_from(2.0_f64) * t652 * t6535;
    let t6539 = t1976 * t671;
    let t6542 = t25 * t776;
    let t6546 = t781 * t154;
    (t6537, t6539, t6542, t6546)
}
