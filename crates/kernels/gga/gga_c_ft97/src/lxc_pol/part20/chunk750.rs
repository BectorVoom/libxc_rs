//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 750/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk750<F: Float>(t17818: F, t24305: F, t232: F, t6054: F, t1609: F, t218: F, t231: F, t2455: F, t6: F, t9681: F, t8: F, t3789: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t24306 = t24305 * t17818;
    let t24307 = t6054 * t232;
    let t24310 = t1609 * sigma2;
    let t24311 = t24310 * t218;
    let t24315 = t231 * t2455;
    let t24322 = t9681 * t6;
    let t24323 = t24322 * t8;
    let t24324 = t3789 * t24323;
    (t24306, t24307, t24310, t24311, t24315, t24322, t24323, t24324)
}
