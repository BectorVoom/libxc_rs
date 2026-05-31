//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 577/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk577<F: Float>(t1594: F, t8035: F, t1631: F, t8031: F, t1711: F, t371: F, t1712: F, t384: F, t374: F, t407: F, t66: F, t428: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8036 = t1594 * t8035;
    let t8039 = t1631 * t8031;
    let t8042 = t371 * t1711;
    let t8043 = t384 * t1712;
    let t8044 = t374 * t8043;
    let t8047 = t1631 * t8035;
    let t8050 = t407 * t407;
    let t8051 = F::cast_from(1.0_f64) / t8050;
    let t8052 = t66 * t8051;
    let t8053 = t1712 * t428;
    (t8036, t8039, t8042, t8044, t8047, t8050, t8051, t8052, t8053)
}
