//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 563/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk563<F: Float>(t3100: F, t360: F, t2124: F, t2545: F, t921: F, t2088: F, t2095: F, t2108: F, t2119: F, t2122: F, t2139: F, t2166: F, t2606: F, t2610: F, t2617: F, t2621: F) -> (F, F, F) {
    let t3101 = t360 * t3100;
    let t3105 = t2124 * t2545 * t921;
    let t3108 = t2088 + t2095 + t2108 + t2119 - F::cast_from(0.97574405393827830186e-2_f64) * t2606 - F::cast_from(0.11643651550782197811e-1_f64) * t2610 + F::cast_from(0.12805040077930161442e0_f64) * t2617 + F::cast_from(0.23115257973478049502e0_f64) * t2621 - t2166 + F::cast_from(0.2600466522016280569e0_f64) * t2139 * t3101 + F::cast_from(0.10975748638225852664e0_f64) * t2122 * t3105;
    (t3101, t3105, t3108)
}
