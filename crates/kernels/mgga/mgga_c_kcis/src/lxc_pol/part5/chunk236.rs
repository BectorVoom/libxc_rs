//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 236/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk236<F: Float>(t157: F, t823: F, t806: F, t810: F, t813: F, t816: F, t819: F) -> (F, F) {
    let t824 = t157 * t823;
    let t826 = t806 / F::new(8.0) - t810 / F::new(8.0) - t813 / F::new(4.0) - t816 / F::new(64.0) + t819 / F::new(64.0) + t824 / F::new(16.0);
    (t824, t826)
}
