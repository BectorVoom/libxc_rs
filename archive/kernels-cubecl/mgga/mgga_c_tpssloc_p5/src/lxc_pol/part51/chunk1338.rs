//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1338/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1338<F: Float>(t31170: F, t5259: F, t5293: F, t5303: F, t114016: F, t5252: F, t1998: F, t5187: F, t59: F, t6926: F, t5287: F, t6936: F, t6943: F) -> (F, F, F, F, F, F) {
    let t120395 = t31170 * t5259;
    let t120397 = t31170 * t5293;
    let t120399 = t31170 * t5303;
    let t120401 = t114016 * t5252;
    let t120405 = t6926 * t1998 * t59 * t5187;
    let t120408 = t6936 * t6943 * t5287;
    (t120395, t120397, t120399, t120401, t120405, t120408)
}
