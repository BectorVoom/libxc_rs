//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1177/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1177<F: Float>(t10470: F, t2180: F, t3245: F, t7732: F, t26972: F, t7768: F, t10466: F, t283: F, t990: F, t26684: F, t61287: F, t14400: F, t2811: F) -> (F, F, F, F, F, F, F) {
    let t93157 = t10470 * t2180;
    let t93158 = F::new(0.51588271604938271604e-3) * t93157;
    let t93163 = t3245 * t7732;
    let t93222 = t7768 * t26972;
    let t93366 = t10466 * t283 * t990;
    let t93425 = t26684 * t61287;
    let t93426 = t14400 * t2811;
    (t93157, t93158, t93163, t93222, t93366, t93425, t93426)
}
