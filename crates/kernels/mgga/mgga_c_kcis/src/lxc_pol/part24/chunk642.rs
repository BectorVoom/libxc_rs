//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 642/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk642<F: Float>(t1142: F, t7766: F, t2192: F, t3638: F, t251: F, t3644: F, t1250: F) -> (F, F, F, F) {
    let t7767 = t1142 * t7766;
    let t7768 = t3638 * t2192;
    let t7771 = t3644 * t251;
    let t7772 = t7771 * t1250;
    (t7767, t7768, t7771, t7772)
}
