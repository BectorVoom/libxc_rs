//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1002/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1002<F: Float>(t1611: F, t218: F, t1410: F, t2427: F, t3758: F, t2432: F, t70: F, t6034: F, t6037: F, t2383: F, t24305: F, t24372: F, t24374: F, t24378: F, t24361: F, t24363: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t96630 = t1611 * sigma2 * t218;
    let t96659 = t2427 * t1410;
    let t96660 = t3758 * t96659;
    let t96690 = t2432 * t70;
    let t96692 = t6034 * t96690 * t6037;
    let t96694 = t2383 * t2427;
    let t96695 = t24305 * t96694;
    let t96700 = t24372 * t24378 * t24374;
    let t96703 = t24361 * t24378 * t24363;
    (t96630, t96659, t96660, t96690, t96692, t96694, t96695, t96700, t96703)
}
