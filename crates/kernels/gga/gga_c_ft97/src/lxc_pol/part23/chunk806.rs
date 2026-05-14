//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 806/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk806<F: Float>(t24330: F, t6043: F, t6046: F, t51: F, t218: F, t6783: F, t1410: F, t695: F, t3758: F, sigma2: F) -> (F, F, F, F) {
    let t24332 = t6043 * t24330 * t6046;
    let t24340 = t51 * sigma2;
    let t24341 = t24340 * t218;
    let t24342 = t6783 * t24341;
    let t24345 = t695 * t1410;
    let t24346 = t3758 * t24345;
    (t24332, t24342, t24345, t24346)
}
