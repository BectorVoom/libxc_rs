//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 883/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk883<F: Float>(t101975: F, t1286: F, t136000: F, t136098: F, t137488: F, t1564: F, t1642: F, t22907: F, t25523: F, t25569: F, t25605: F, t25611: F, t25617: F, t26128: F, t28: F, t32355: F, t34358: F, t34362: F, t378: F, t5495: F, t5501: F, t5507: F, t7166: F, t7212: F, t925: F) -> (F,) {
    let t144613 = -t5501 * t1564 * t136000 * t925 / 18.0 - 2.0 / 3.0 * t1286 * t28 * t5507 * t101975 + t136098 / 9.0 - t1286 * t28 * t32355 * t25523 / 3.0 - 2.0 / 3.0 * t5495 * t34362 - t1286 * t28 * t32355 * t26128 / 3.0 - t5495 * t34358 / 3.0 + t5501 * t378 * t7212 * t25611 / 9.0 - t5501 * t1642 * t7212 * t25617 / 27.0 - t5501 * t137488 * t25605 / 3.0 + 2.0 / 9.0 * t5501 * t22907 * t25569 - 2.0 / 9.0 * t5501 * t378 * t7166 * t25611 + 2.0 / 27.0 * t5501 * t1642 * t7166 * t25617;
    (t144613,)
}
