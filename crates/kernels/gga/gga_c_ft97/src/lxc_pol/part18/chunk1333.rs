//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1333/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1333<F: Float>(t105534: F, t446: F, t9073: F, t26768: F, t558: F, t1369: F, t2112: F, t28: F, t1637: F, t5890: F, t6657: F, t3000: F, t586: F, t5916: F, t2075: F, t6615: F) -> (F, F, F, F, F, F, F) {
    let t105703 = t446 * t9073 * t105534;
    let t105705 = t26768 * t558;
    let t105708 = t1369 * t28 * t2112 * t105705;
    let t105711 = t5890 * t1637 * t6657;
    let t105712 = t105711 / 9.0;
    let t105715 = t1369 * t3000 * t586 * t5916;
    let t105717 = t6615 * t2075;
    (t105703, t105705, t105708, t105711, t105712, t105715, t105717)
}
