//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1112/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1112<F: Float>(t43: F, t11698: F, t11700: F, t11702: F, t11704: F, t14919: F, t11607: F, t1361: F, t14810: F, t1690: F, t1694: F, t19461: F, t234: F, t2861: F, t2868: F, t35: F, t3996: F, t5445: F, t5450: F, t5455: F, t595: F, t818: F, t821: F, t824: F, t886: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t19914 = F::new(192.0) * t11698;
    let t19915 = F::new(24.0) * t11700;
    let t19916 = F::new(64.0) * t11702;
    let t19917 = F::cast_from(0.11696447245269292414e1_f64) * t11704;
    let t19918 = F::new(32.0) * t14919;
    let t19942 = piecewise3::<F>(t44, F::new(0.0), F::new(40.0) / F::new(81.0) * t11607 * t1690 * t818 - F::new(64.0) / F::new(27.0) * t3996 * t19461 - F::new(8.0) / F::new(27.0) * t5445 * t824 + F::new(32.0) / F::new(9.0) * t886 * t35 * t595 + F::new(16.0) / F::new(9.0) * t1361 * t821 - F::new(16.0) / F::new(3.0) * t1361 * t2868 - F::new(8.0) / F::new(27.0) * t2861 * t1694 * t818 + F::new(8.0) / F::new(9.0) * t886 * t5455 * t234 + F::new(4.0) / F::new(9.0) * t5450 * t824 + t14810);
    (t19914, t19915, t19916, t19917, t19918, t19942)
}
