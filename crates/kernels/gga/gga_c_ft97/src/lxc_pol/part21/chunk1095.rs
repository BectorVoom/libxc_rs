//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1095/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1095<F: Float>(t40424: F, t5900: F, t92: F, t95262: F, t6662: F, t95099: F, t1369: F, t1637: F, t6665: F, t27185: F, t376: F, t89: F, t27087: F, t5890: F, t27174: F, t24: F, t9236: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t105579 = t40424 * t5900;
    let t105592 = t95262 * t92;
    let t105598 = t95099 * t6662;
    let t105617 = t1369 * t1637 * t6665;
    let t105637 = t89 * t376 * t27185;
    let t105638 = 4.0 / 3.0 * t105637;
    let t105671 = t5890 * t376 * t27087;
    let t105672 = t105671 / 6.0;
    let t105677 = t89 * t376 * t27174;
    let t105678 = 4.0 / 3.0 * t105677;
    let t105679 = t24 * t9236;
    (t105579, t105592, t105598, t105617, t105637, t105638, t105671, t105672, t105677, t105678, t105679)
}
