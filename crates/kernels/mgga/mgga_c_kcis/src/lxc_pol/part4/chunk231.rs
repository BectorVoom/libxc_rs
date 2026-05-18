//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 231/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk231<F: Float>(t728: F, t794: F, t11: F, t122: F, t144: F, t145: F, t148: F, t745: F, t784: F, t788: F, t791: F, t85: F) -> (F, F) {
    let t795 = t794 * t728;
    let t804 = F::new(0.619125e-2) * t784 * t145 - F::new(0.123825e-1) * t788 * t791 - F::new(0.619125e-2) * t144 * t795 - F::new(0.53062222222222222221e-1) * t85 * t11 * t122 - F::new(0.79593333333333333331e-1) * t85 * t148 * t745;
    (t795, t804)
}
