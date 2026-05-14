//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1006/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1006<F: Float>(t1786: F, t4545: F, t16533: F, t487: F, t370: F, t971: F, t3238: F, t463: F, t1780: F, t8216: F, t986: F, t1587: F, t1852: F, t10969: F, t1851: F, t8326: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t59510 = t1786 * t4545;
    let t59565 = t16533 * t487;
    let t59631 = t370 * t971;
    let t59659 = t463 * t3238;
    let t59663 = t1780 * t3238;
    let t60243 = t8216 * t986;
    let t60426 = t1587 * t1852;
    let t60711 = t463 * t10969;
    let t60805 = t1786 * t986;
    let t60901 = t1587 * t971;
    let t61025 = t4545 * t1851;
    let t61053 = t8326 * t986;
    (t59510, t59565, t59631, t59659, t59663, t60243, t60426, t60711, t60805, t60901, t61025, t61053)
}
