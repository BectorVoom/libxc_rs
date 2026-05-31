//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1067/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1067<F: Float>(t1485: F, t1508: F, t1531: F, t1499: F, t126: F, t82: F, t94: F, t98: F, t5075: F, t512: F, t83: F, t1511: F, t5336: F) -> (F, F, F, F, F) {
    let t16886 = F::cast_from(0.12842595503380418954e1_f64) * t1531 * t1485 * t1508;
    let t16889 = F::cast_from(0.43374325201206959368e-1_f64) * t1531 * t1485 * t1499;
    let t16893 = F::cast_from(24.0_f64) * t82 * t94 * t98 * t126;
    let t16897 = t83 * t512 * t5075;
    let t16901 = t1511 * t5336;
    (t16886, t16889, t16893, t16897, t16901)
}
