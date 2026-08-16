//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1113/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1113<F: Float>(t33191: F, t1458: F, t8326: F, t3941: F, t1441: F, t1873: F, t12571: F, t8662: F, t7266: F, t7468: F, t2113: F) -> (F, F, F, F, F, F, F) {
    let t33192 = F::cast_from(0.135e2_f64) * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = F::cast_from(27.0_f64) * t33194;
    let t33211 = t1441 * t1873;
    let t33669 = t12571 * t8662;
    let t33688 = t7266 * t7468;
    let t33690 = t2113 * t1458;
    (t33192, t33193, t33195, t33211, t33669, t33688, t33690)
}
