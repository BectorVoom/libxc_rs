//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 454/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk454<F: Float>(t2880: F, t995: F, t991: F, t1004: F, t25: F, t285: F, t335: F) -> (F, F, F, F, F) {
    let t2881 = t2880 * t995;
    let t2882 = t991 * t2881;
    let t2884 = t25 * t1004;
    let t2885 = t285 * t2884;
    let t2887 = F::cast_from(1.0_f64) / t335;
    (t2881, t2882, t2884, t2885, t2887)
}
