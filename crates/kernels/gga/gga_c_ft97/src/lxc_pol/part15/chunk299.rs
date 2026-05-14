//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 299/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk299<F: Float>(t407: F, t76: F, t66: F, t47: F, t625: F, t68: F, t72: F, t23: F, t358: F) -> (F, F, F, F, F) {
    let t1710 = 1.0 / t407 / t76;
    let t1711 = t66 * t1710;
    let t1728 = t47 * t625;
    let t1730 = t68 * t1728 * t72;
    let t1731 = 0.42562405586419753087e-2 * t1730;
    let t1736 = 1.0 / t23 / t358;
    (t1710, t1711, t1730, t1731, t1736)
}
