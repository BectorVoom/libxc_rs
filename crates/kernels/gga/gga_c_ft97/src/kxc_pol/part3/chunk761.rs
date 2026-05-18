//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 761/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk761<F: Float>(t15860: F, t419: F, t1725: F, t4488: F, t173: F, t4487: F, t15763: F, t3088: F, t1527: F, t15768: F, t15625: F, t423: F) -> (F, F, F, F, F, F) {
    let t15861 = t419 * t15860;
    let t15863 = t1725 * t4488;
    let t15865 = t173 * t4487;
    let t15866 = t419 * t15865;
    let t15868 = t3088 * t15763;
    let t15869 = t419 * t15868;
    let t15871 = t1527 * t15768;
    let t15872 = t419 * t15871;
    let t15874 = t423 * t15625;
    (t15861, t15863, t15866, t15869, t15872, t15874)
}
