//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 296/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk296<F: Float>(t1755: F, t370: F, t27: F, t89: F, t1545: F, t1549: F, t1552: F, t1562: F, t1567: F, t1574: F, t1583: F, t1591: F, t348: F, t95: F, t96: F) -> (F, F, F, F) {
    let t1756 = t370 * t1755;
    let t1758 = t89 * t27 * t1756;
    let t1760 = t1545 + t1549 + t1552 - t1562 / 27.0 + t1567 / 9.0 + t1574 / 9.0 - t1583 / 18.0 + t1591 / 3.0 - t1758 / 6.0;
    let t1761 = t348 * t1760;
    let t1766 = 1.0 / t96 / t95;
    (t1756, t1758, t1761, t1766)
}
