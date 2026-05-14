//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1234/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1234<F: Float>(t5812: F, t77143: F, t115984: F, t5838: F, t22632: F, t30075: F, t5829: F, t104788: F, t104792: F, t115981: F, t116029: F, t116033: F, t118789: F, t16786: F, t22767: F, t23701: F, t23839: F, t26692: F, t5557: F, t5613: F) -> (F,) {
    let t118795 = t77143 * t5812;
    let t118804 = t5838 * t115984;
    let t118810 = t5829 * t22632 * t30075;
    let t118814 = -0.24167761770734866964e0 * t23839 * t118789 - 0.33339000546296296297e-1 * t118795 * t5613 + 0.80559205902449556551e-1 * t23701 * t116029 + 0.13335600218518518519e0 * t26692 * t116033 + 0.59269334304526748973e-1 * t5838 * t115981 - 0.74086667880658436217e-2 * t118804 - 0.26671200437037037037e0 * t5829 * t22767 * t30075 + 0.33339000546296296297e-1 * t118810 + t104788 + t104792 + 0.21895580739717983994e1 * t16786 * t5557;
    (t118814,)
}
