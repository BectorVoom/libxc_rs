//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 595/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk595<F: Float>(t237: F, t27703: F, t17859: F, t24260: F, t1100: F, t1416: F, t1113: F, t218: F, t709: F, t24345: F) -> (F, F, F, F, F) {
    let t27704 = t27703 * t237;
    let t27707 = t24260 * t17859;
    let t27711 = t1100 * t1416;
    let t27712 = t218 * t1113;
    let t27713 = t27712 * t709;
    let t27717 = t1100 * t24345;
    (t27704, t27707, t27711, t27713, t27717)
}
