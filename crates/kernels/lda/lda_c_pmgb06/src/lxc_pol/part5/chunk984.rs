//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 984/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk984<F: Float>(t12006: F, t2623: F, t493: F, t529: F, t851: F, t10711: F, t10714: F, t13088: F, t20451: F, t20452: F, t20454: F, t20456: F, t20460: F, t20463: F, t20465: F, t20467: F) -> (F, F) {
    let t20472 = 3.0 / 5.0 * t493 * t12006 * t2623 * t851 * t529;
    let t20473 = t20451 + t13088 - t20452 - t20454 + t10711 + t10714 - t20456 - t20460 + t20463 - t20465 - t20467 - t20472;
    (t20472, t20473)
}
