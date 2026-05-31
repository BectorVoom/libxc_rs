//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 497/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk497<F: Float>(t35: F, t401: F, t1711: F, t6: F, t64: F, t62: F, t66: F, t371: F, t407: F, t1693: F, t1586: F, t355: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7878 = t35 * t401;
    let t7888 = t1711 * t6;
    let t7889 = t64 * t7888;
    let t7983 = t62 * t66;
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = F::cast_from(1.0_f64) / t8050;
    let t8052 = t66 * t8051;
    let t8155 = t1693 * t1693;
    let t8216 = t355 * t1586;
    (t7878, t7888, t7889, t7983, t8042, t8050, t8051, t8052, t8155, t8216)
}
