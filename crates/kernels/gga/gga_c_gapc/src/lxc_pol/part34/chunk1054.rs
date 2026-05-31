//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1054/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1054<F: Float>(t3449: F, t3565: F, t3832: F, t7056: F, t11046: F, t3268: F, t3265: F, t3622: F, t11614: F, t11617: F, t11621: F, t11623: F, t11627: F, t11630: F, t11634: F, t11638: F, t11641: F, t11644: F, t11649: F, t11651: F, t11653: F) -> (F, F, F, F, F) {
    let t12156 = t3565 * t3449;
    let t12158 = F::cast_from(2.0_f64) * t7056 * t3832;
    let t12161 = F::cast_from(2.0_f64) * t11046 * t3268;
    let t12162 = t3265 * t3622;
    let t12176 = -F::cast_from(0.16414765573575218917e-4_f64) * t11614 - F::cast_from(0.16414765573575218917e-4_f64) * t11617 + F::cast_from(0.23485962392041415794e-5_f64) * t11621 - F::cast_from(0.16146599144528473358e-4_f64) * t11623 + F::cast_from(0.23485962392041415794e-4_f64) * t11627 + F::cast_from(0.14678726495025884871e-5_f64) * t11630 + F::cast_from(0.86995919027186744337e-7_f64) * t11634 + F::cast_from(0.14678726495025884871e-5_f64) * t11638 + F::cast_from(0.23485962392041415794e-4_f64) * t11641 - F::cast_from(0.34197428278281706076e-6_f64) * t11644 - F::cast_from(0.99742499144988309388e-7_f64) * t11649 + F::cast_from(0.30777685450453535468e-5_f64) * t11651 + F::cast_from(0.93943849568165663176e-4_f64) * t11653;
    (t12156, t12158, t12161, t12162, t12176)
}
