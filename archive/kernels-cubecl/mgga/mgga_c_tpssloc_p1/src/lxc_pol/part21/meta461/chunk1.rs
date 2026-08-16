//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2027/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2027<F: Float>(t1372: F, t1824: F, t5250: F, t5286: F, t562: F, t3851: F, t5335: F, t12248: F, t68: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t16036 = t1372 * t1824;
    let t16037 = t16036 * t5250;
    let t16040 = t562 * t5286;
    let t16041 = t16040 * t5250;
    let t16044 = t5335 * t3851;
    let t16046 = t68 * t12248;
    let t16047 = t544 * t16046;
    (t16036, t16037, t16040, t16041, t16044, t16046, t16047)
}
