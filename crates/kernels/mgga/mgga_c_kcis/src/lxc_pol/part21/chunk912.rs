//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 912/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk912<F: Float>(t2933: F, t4719: F, t1670: F, t9752: F, t2944: F, t2960: F, t4625: F, t934: F, t2952: F, t4700: F, t287: F, t330: F) -> (F, F, F, F, F) {
    let t13878 = F::cast_from(2.0_f64) * t2933 * t4719;
    let t13880 = t9752 * t1670;
    let t13881 = t13880 * t2944;
    let t13885 = t2960 * t4625;
    let t13886 = t13885 * t934;
    let t13888 = t4700 * t2952;
    let t13890 = t287 * t330;
    (t13878, t13881, t13886, t13888, t13890)
}
