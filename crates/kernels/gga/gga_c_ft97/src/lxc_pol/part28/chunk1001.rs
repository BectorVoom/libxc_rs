//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1001/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1001<F: Float>(t27142: F, t3052: F, t32924: F, t9073: F, t148288: F, t446: F, t34822: F, t558: F, t9432: F, t1369: F, t147590: F, t28: F, t586: F, t139509: F, t1969: F, t920: F) -> (F, F, F, F, F) {
    let t148545 = t27142 * t9073 * t32924 * t3052;
    let t148551 = t446 * t9073 * t148288;
    let t148555 = t446 * t9432 * t34822 * t558;
    let t148559 = t1369 * t28 * t586 * t147590;
    let t148563 = t446 * t1969 * t139509 * t920;
    (t148545, t148551, t148555, t148559, t148563)
}
