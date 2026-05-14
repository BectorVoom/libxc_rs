//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 859/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk859<F: Float>(t1904: F, t23265: F, t8557: F, t1911: F, t11854: F, t23085: F, t83: F, t22946: F, t22948: F, t1882: F, t5728: F, t22944: F, t22941: F, t23093: F, t487: F, t5743: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23266 = t23265 * t1904;
    let t23267 = t8557 * t23266;
    let t23270 = t23265 * t1911;
    let t23271 = t11854 * t23270;
    let t23274 = t83 * t23085;
    let t23277 = t83 * t22946;
    let t23280 = t83 * t22948;
    let t23283 = t1882 * t5728;
    let t23285 = t83 * t22944;
    let t23288 = t83 * t22941;
    let t23291 = t83 * t23093;
    let t23294 = t487 * t5743;
    (t23266, t23267, t23270, t23271, t23274, t23277, t23280, t23283, t23285, t23288, t23291, t23294)
}
