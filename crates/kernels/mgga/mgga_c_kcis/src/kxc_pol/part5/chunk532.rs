//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 532/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk532<F: Float>(t2626: F, t783: F, t171: F, t167: F, t740: F) -> (F, F, F, F) {
    let t2627 = t783 * t2626;
    let t2628 = t171 * t171;
    let t2629 = F::new(1.0) / t2628;
    let t2633 = t167 * t740;
    (t2627, t2628, t2629, t2633)
}
