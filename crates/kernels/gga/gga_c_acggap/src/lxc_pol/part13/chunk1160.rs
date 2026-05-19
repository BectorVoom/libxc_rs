//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1160/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1160<F: Float>(t31629: F, t31646: F, t1426: F, t429: F, t598: F, t8539: F, t35500: F, t7380: F, t34050: F, t2095: F, t33901: F, t33884: F) -> (F, F, F, F, F, F, F) {
    let t35898 = F::cast_from(0.12862205435420921092e-1_f64) * t31629;
    let t35904 = F::cast_from(0.32012600194825403606e-1_f64) * t31646;
    let t35907 = t598 * t1426 * t429 * t8539;
    let t35909 = t7380 * t35500;
    let t35910 = F::new(0.4584375e-1) * t35909;
    let t35911 = t7380 * t34050;
    let t35912 = F::new(0.4584375e-1) * t35911;
    let t35913 = t2095 * t33901;
    let t35914 = F::new(0.305625e-1) * t35913;
    let t35915 = t2095 * t33884;
    (t35898, t35904, t35907, t35910, t35912, t35914, t35915)
}
