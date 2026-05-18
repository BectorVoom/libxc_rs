//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 608/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk608<F: Float>(t1241: F, t1249: F, t1259: F, t1274: F, t1280: F, t2215: F, t2227: F, t2694: F, t2698: F, t2701: F, t2704: F, t2708: F, t2712: F, t2715: F, t360: F, t63: F) -> F {
    let t2718 = -t1241 + t2694 + t1249 + t2698 - t2701 + t1259 + t2215 / F::new(3.0) + F::new(3.0) / F::new(2.0) * t360 * t2704 - t360 * t2708 / F::new(2.0) + t1274 + F::new(1.46904) * t2227 + t1280 + F::new(5.87616) * t63 * t2712 - F::new(1.46904) * t63 * t2715;
    t2718
}
