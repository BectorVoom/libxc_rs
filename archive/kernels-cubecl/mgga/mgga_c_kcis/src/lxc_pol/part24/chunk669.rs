//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 669/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk669<F: Float>(t1203: F, t7740: F, t2189: F, t3325: F, t3330: F, t1165: F, t377: F, t1169: F, t283: F) -> (F, F, F, F, F, F) {
    let t7741 = t7740 * t1203;
    let t7742 = t3325 * t2189;
    let t7743 = t2189 * t1203;
    let t7745 = F::cast_from(2.0_f64) * t3330 * t7743;
    let t7746 = t1165 * t377;
    let t7748 = t1169 * t283;
    (t7741, t7742, t7743, t7745, t7746, t7748)
}
