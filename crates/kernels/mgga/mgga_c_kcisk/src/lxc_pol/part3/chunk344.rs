//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 344/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk344<F: Float>(t1224: F, t1636: F, t1697: F, t1696: F, t617: F, t608: F, t609: F, t1695: F) -> (F, F, F, F, F, F, F) {
    let t1699 = t1224 * t1697 * t1636;
    let t1701 = -t1696 - F::cast_from(0.17808333333333333333e-1_f64) * t1699;
    let t1704 = t617 * t617;
    let t1705 = F::new(1.0) / t1704;
    let t1706 = t608 * t1705;
    let t1707 = F::new(1.0) / t609;
    let t1709 = -t1695 / F::new(3.0) - t1699 / F::new(3.0);
    (t1699, t1701, t1704, t1705, t1706, t1707, t1709)
}
