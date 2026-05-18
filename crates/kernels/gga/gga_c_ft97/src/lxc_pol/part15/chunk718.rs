//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 718/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk718<F: Float>(t20542: F, t446: F, t1017: F, t4454: F, t9049: F, t20022: F, t9054: F, t1555: F, t89: F, t20031: F, t2205: F, t4462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20543 = t446 * t20542;
    let t20545 = t4454 * t1017;
    let t20546 = t9049 * t20545;
    let t20547 = t446 * t20546;
    let t20549 = t9054 * t20022;
    let t20551 = t89 * t1555 * t20549;
    let t20553 = t2205 * t20031;
    let t20554 = t446 * t20553;
    let t20556 = t4462 * t1017;
    (t20543, t20545, t20546, t20547, t20549, t20551, t20553, t20554, t20556)
}
