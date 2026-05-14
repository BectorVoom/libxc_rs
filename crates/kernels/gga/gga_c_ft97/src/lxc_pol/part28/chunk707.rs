//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 707/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk707<F: Float>(t86: F, t32390: F, t32649: F, t113: F, t5: F, t505: F, t7293: F, t5764: F, t7150: F, t1374: F, t1774: F, t7298: F, t1360: F, t379: F, t356: F, t461: F, t5925: F) -> (F, F, F, F, F, F, F, F) {
    let t87 = 10000000.0 <= t86;
    let t32650 = t32390 + t32649;
    let t32657 = piecewise3(t87, 0.0, t5 * t32650 * t113 / 4.0 + t5 * t7293 * t505 / 4.0);
    let t32658 = t5764 * t7150;
    let t32661 = t1774 * t1374;
    let t32663 = t7298 * t32661 / 18.0;
    let t32664 = t1360 * t379;
    let t32665 = t356 * t32664;
    let t32670 = t461 * t5925;
    (t32650, t32657, t32658, t32661, t32663, t32664, t32665, t32670)
}
