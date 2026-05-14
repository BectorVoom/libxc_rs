//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1338/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1338<F: Float>(t105353: F, t446: F, t9049: F, t105341: F, t39725: F, t1986: F, t6615: F, t1369: F, t28: F, t9236: F, t11604: F, t23892: F, t1969: F, t3281: F, t105493: F, t23652: F, t27157: F, t27158: F, t574: F) -> (F, F, F, F, F, F, F, F) {
    let t105776 = t446 * t9049 * t105353;
    let t105779 = t446 * t39725 * t105341;
    let t105781 = t6615 * t1986;
    let t105784 = t1369 * t28 * t9236 * t105781;
    let t105786 = t23892 * t11604;
    let t105788 = t3281 * t1969 * t105786;
    let t105791 = t3281 * t9049 * t105493;
    let t105795 = t27157 * t574 * t23652 * t27158;
    (t105776, t105779, t105781, t105784, t105786, t105788, t105791, t105795)
}
