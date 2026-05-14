//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 820/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk820<F: Float>(t3328: F, t377: F, t947: F, t1036: F, t1095: F, t1131: F, t398: F, t864: F, t3476: F, t957: F, t3371: F, t3378: F) -> (F, F, F, F, F) {
    let t12854 = t377 * t3328;
    let t12855 = t12854 * t947;
    let t12862 = t1036 * t398 * t1095 * t1131 * t864;
    let t12899 = t3476 * t957;
    let t12930 = t3378 * t3371;
    (t12854, t12855, t12862, t12899, t12930)
}
