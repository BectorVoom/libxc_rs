//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 528/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk528<F: Float>(t4: F, t66: F, t789: F, t128: F, t717: F, t2440: F, t2459: F, t794: F, t103: F, t11: F, t122: F, t144: F, t145: F, t148: F, t2477: F, t2542: F, t2546: F, t2552: F, t2555: F, t2559: F, t2561: F, t745: F, t784: F, t788: F, t791: F, t795: F, t85: F) -> (F, F, F, F, F) {
    let t2565 = t789 * t4 * t66;
    let t2568 = t128 * t717;
    let t2569 = t2568 * t2440;
    let t2572 = t794 * t2459;
    let t2584 = F::new(0.619125e-2) * t2542 * t145 - F::new(0.24765e-1) * t2546 * t791 - F::new(0.123825e-1) * t784 * t795 + F::new(0.206375e-2) * t2552 * t2555 + F::new(0.24765e-1) * t2559 * t2561 + F::new(0.1651e-1) * t788 * t2565 + F::new(0.123825e-1) * t144 * t2569 - F::new(0.619125e-2) * t144 * t2572 + F::cast_from(0.17687407407407407407e-1_f64) * t85 * t103 * t122 - F::cast_from(0.10612444444444444444e0_f64) * t85 * t11 * t745 - F::cast_from(0.79593333333333333331e-1_f64) * t85 * t148 * t2477;
    (t2565, t2568, t2569, t2572, t2584)
}
