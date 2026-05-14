//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1217/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1217<F: Float>(t10406: F, t30575: F, t30578: F, t30607: F, t30629: F, t30631: F, t30633: F, t30644: F, t30647: F, t30650: F, t34431: F, t34435: F, t34436: F, t34442: F, t34445: F, t34449: F, t4849: F) -> (F,) {
    let t34450 = t34431 + t34435 - t34436 - t30575 + t30578 - 0.1022478025437886658e1 * t4849 * t10406 + t34442 + t30607 + t30629 - t30631 + t30633 + t30644 - t30647 + t30650 - t34445 - t34449;
    (t34450,)
}
