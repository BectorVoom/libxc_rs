//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 397/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk397<F: Float>(t2440: F, t717: F, t2459: F, t97: F, t684: F, t127: F, t129: F, t130: F, t2379: F, t2496: F, t2500: F, t2507: F, t2508: F, t60: F, t756: F, t763: F, t764: F, t768: F) -> (F, F, F) {
    let t2514 = t717 * t2440;
    let t2518 = t97 * t2459;
    let t2522 = t684 * t684;
    let t2526 = -F::cast_from(0.43802864444444444443e-3_f64) * t127 * t2496 * t130 - F::new(0.2e-22) * t763 * t2500 * t130 - F::cast_from(0.26281718666666666666e-2_f64) * t127 * t756 * t768 + F::cast_from(0.19711288999999999999e-2_f64) * t2507 * t2508 + F::cast_from(0.19711288999999999999e-2_f64) * t763 * t764 * t768 + F::cast_from(0.39422577999999999998e-2_f64) * t127 * t129 * t2514 - F::cast_from(0.19711288999999999999e-2_f64) * t127 * t129 * t2518 - F::new(4.0) * t2522 - F::new(4.0) * t60 * t2379;
    (t2514, t2518, t2526)
}
