//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 616/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk616<F: Float>(t1863: F, t1882: F, t1825: F, t1876: F, t452: F, t104: F, t7943: F, t89: F, t3187: F, t8376: F, t1909: F, t1588: F, t492: F) -> (F, F, F, F, F, F) {
    let t8526 = t1882 * t1863;
    let t8529 = t452 * t1825 * t1876;
    let t8534 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t89 * t7943 * t104;
    let t8535 = t3187 * t8376;
    let t8536 = t1909 * t8535;
    let t8539 = t1588 * t492;
    (t8526, t8529, t8534, t8535, t8536, t8539)
}
