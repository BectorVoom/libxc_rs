//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 425/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk425<F: Float>(t1608: F, t1610: F, t286: F, t1597: F, t1599: F, t1603: F, t622: F) -> (F, F, F, F) {
    let t1611 = t1608 * t1610;
    let t1612 = t286 * t1611;
    let t1615 = t1597 + t1599 * t1603 / F::new(576.0) - t1599 * t1612 / F::new(192.0);
    let t1616 = F::new(1.0) / t622;
    (t1611, t1612, t1615, t1616)
}
