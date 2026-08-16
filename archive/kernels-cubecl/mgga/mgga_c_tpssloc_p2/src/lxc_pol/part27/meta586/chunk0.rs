//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2041/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2041<F: Float>(t22989: F, t81591: F, t22690: F, t23153: F, t23171: F, t6561: F, t80741: F, t6643: F, t23025: F, t23030: F, t23012: F, t6653: F) -> (F, F, F, F, F, F) {
    let t81592 = t81591 * t22989;
    let t81595 = t23171 * t22690 * t23153;
    let t81597 = t80741 * t6561;
    let t81598 = t81597 * t6643;
    let t81599 = F::cast_from(0.16220877603642232915e0_f64) * t81598;
    let t81600 = t23030 * t23025;
    let t81602 = t23012 * t6653;
    (t81592, t81595, t81597, t81599, t81600, t81602)
}
