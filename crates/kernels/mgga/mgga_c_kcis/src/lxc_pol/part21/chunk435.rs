//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 435/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk435<F: Float>(t169: F, t980: F, t2635: F, t234: F, t1008: F, sigma0: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t170 = t169 <= zeta_threshold;
    let t2793 = F::cast_from(1.0_f64) / t980;
    let t2794 = sigma0 * t2793;
    let t2801 = piecewise3::<F>(t170, F::cast_from(0.0_f64), t2635);
    let t2802 = t234 * t2801;
    let t2809 = t1008 * t1008;
    (t2793, t2794, t2801, t2802, t2809)
}
