//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 998/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk998<F: Float>(t231: F, t5005: F, t6045: F, t5049: F, t7462: F, t11: F, t14: F, t6820: F, t30696: F, t41: F, t4995: F, t237: F, t1100: F, t5009: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30717 = t231 * t5005;
    let t30718 = t6045 * t30717;
    let t30721 = t231 * t5049;
    let t30725 = 1.0 / t7462;
    let t30726 = t11 * t30725;
    let t30727 = t30726 * t14;
    let t30728 = t30727 * t6820;
    let t30756 = 0.44057546758024691357e0 * t41 * t11 * t4995 + 0.37540077436335915589e-1 * t30696;
    let t30757 = t237 * t30756;
    let t30758 = t1100 * t30757;
    let t30760 = t5009 * sigma2;
    (t30717, t30718, t30721, t30725, t30726, t30727, t30728, t30756, t30758, t30760)
}
