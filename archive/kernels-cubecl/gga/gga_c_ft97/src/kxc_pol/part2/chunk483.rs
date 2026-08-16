//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 483/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk483<F: Float>(t2801: F, t871: F, t296: F, t824: F, t840: F, t882: F, t2739: F, t319: F, t2399: F, t313: F, t89: F, t1882: F, t842: F) -> (F, F, F, F, F, F) {
    let t2802 = t871 * t2801;
    let t2803 = t296 * t2802;
    let t2807 = t840 * t882 * t824;
    let t2811 = t840 * t319 * t2739;
    let t2816 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t2399 * t313;
    let t2817 = t1882 * t842;
    (t2802, t2803, t2807, t2811, t2816, t2817)
}
