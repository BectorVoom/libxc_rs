//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 945/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk945<F: Float>(t3500: F, t7790: F, t7788: F, t251: F, t3638: F, t1250: F) -> (F, F, F, F) {
    let t27006 = t3500 * t7790;
    let t27007 = t7788 * t27006;
    let t27013 = t3638 * t251;
    let t27014 = t27013 * t1250;
    (t27006, t27007, t27013, t27014)
}
