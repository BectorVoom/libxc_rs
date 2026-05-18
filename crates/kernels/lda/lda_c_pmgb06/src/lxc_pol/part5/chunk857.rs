//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 857/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk857<F: Float>(t1121: F, t410: F, t980: F, t110: F, t3705: F, t1180: F, t698: F, t1025: F, t3675: F, t3868: F, t633: F, t964: F) -> (F, F, F, F, F, F, F) {
    let t8583 = F::new(0.08674865040241392) * t1121 * t410 * t980;
    let t8586 = F::new(0.13012297560362088) * t1121 * t110 * t3705;
    let t8589 = F::new(0.06747117253521083) * t1121 * t1180 * t698;
    let t8594 = F::new(578.9512619529313) * t3675 * t3868 * t1025;
    let t8595 = t1025 * t1025;
    let t8598 = F::new(24.0) * t3675 * t8595 * t633;
    let t8599 = t964 * t964;
    (t8583, t8586, t8589, t8594, t8595, t8598, t8599)
}
