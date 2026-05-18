//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1264/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1264<F: Float>(t16120: F, t3902: F, t11571: F, t1907: F, t3856: F, t5574: F, t13948: F, t5570: F, t1903: F, t2331: F, t11491: F, t1897: F) -> (F, F, F, F, F, F) {
    let t16122 = F::new(0.16081824322151104822e2) * t16120 * t3902;
    let t16124 = F::new(1.0) * t11571 * t1907;
    let t16126 = F::new(2.0) * t3856 * t5574;
    let t16127 = t13948 * t5570;
    let t16129 = t2331 * t1903;
    let t16131 = t11491 * t1897;
    (t16122, t16124, t16126, t16127, t16129, t16131)
}
