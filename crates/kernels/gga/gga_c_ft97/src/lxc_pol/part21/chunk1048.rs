//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1048/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1048<F: Float>(t100292: F, t23054: F, t25934: F, t11176: F, t1316: F, t25880: F, t25875: F, t1882: F, t25972: F, t25975: F, t25978: F, t1900: F, t457: F, t6: F, t91: F, t38463: F, t5675: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t100293 = t100292 / 12.0;
    let t100294 = t23054 * t25934;
    let t100295 = 2.0 / 27.0 * t100294;
    let t100305 = t1316 * t11176 * t25880;
    let t100307 = t23054 * t25875;
    let t100308 = t100307 / 27.0;
    let t100309 = t1882 * t25972;
    let t100310 = 4.0 / 27.0 * t100309;
    let t100311 = t1882 * t25975;
    let t100312 = 4.0 / 27.0 * t100311;
    let t100313 = t1882 * t25978;
    let t100314 = 4.0 / 81.0 * t100313;
    let t100356 = t91 * t457 * t6 * t1900;
    let t100360 = t38463 * t5675;
    (t100293, t100294, t100295, t100305, t100307, t100308, t100309, t100310, t100311, t100312, t100313, t100314, t100356, t100360)
}
