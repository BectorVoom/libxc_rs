//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1414/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1414<F: Float>(t174: F, t740: F, t9323: F, t447: F, t637: F, t446: F, t1295: F, t4534: F, t233: F, t1655: F, t2791: F, t5399: F, t911: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t175 = t174 <= zeta_threshold;
    let t18374 = F::new(2.0) * t740;
    let t18375 = F::new(6.0) * t9323;
    let t18376 = -t18374 + t18375;
    let t18377 = piecewise3::<F>(t175, F::new(0.0), t18376);
    let t18378 = t447 * t18377;
    let t18379 = t18378 * t637;
    let t18380 = t446 * t18379;
    let t18382 = t4534 * t1295;
    let t18383 = t233 * t18382;
    let t18385 = t1655 * t2791;
    let t18386 = t911 * t5399;
    (t18376, t18380, t18383, t18385, t18386)
}
