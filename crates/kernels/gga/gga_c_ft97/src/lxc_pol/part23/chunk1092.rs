//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1092/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1092<F: Float>(t24322: F, t3771: F, t5567: F, t229: F, t9: F, t6056: F, t13580: F, t6019: F, t6043: F, t6046: F, t96535: F, t1611: F, t218: F, t1410: F, t2427: F, t3758: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t96607 = t3771 * t24322 * t5567;
    let t96615 = t9 * t229;
    let t96616 = t96615 * t6056;
    let t96619 = t13580 * t6019;
    let t96623 = t6043 * t96535 * t6046;
    let t96630 = t1611 * sigma2 * t218;
    let t96659 = t2427 * t1410;
    let t96660 = t3758 * t96659;
    (t96607, t96615, t96616, t96619, t96623, t96630, t96660)
}
