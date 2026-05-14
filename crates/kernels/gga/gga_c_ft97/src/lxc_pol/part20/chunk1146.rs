//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1146/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1146<F: Float>(t1882: F, t28441: F, t28158: F, t46862: F, t1131: F, t11593: F, t13885: F, t13938: F, t13943: F, t14012: F, t14087: F, t14140: F, t14213: F, t1424: F, t1901: F, t24564: F, t24737: F, t24793: F, t28129: F, t28136: F, t28140: F, t28360: F, t28404: F, t446: F, t51853: F, t53942: F, t6161: F, t729: F, t762: F, t97488: F, t97490: F, t97492: F, t9787: F) -> (F,) {
    let t110293 = 4.0 / 9.0 * t1882 * t28441;
    let t110294 = t46862 * t28158;
    let t110316 = t446 * t729 * t762 * t24564 * t1131 / 3.0 - 2.0 / 3.0 * t1901 * t13885 * t6161 * t14213 + 2.0 * t1901 * t28140 * t24737 * t14140 - 2.0 / 27.0 * t97488 - 2.0 / 27.0 * t97490 + 2.0 / 27.0 * t97492 - 4.0 / 9.0 * t11593 * t9787 * t28360 - t110293 + 22.0 / 27.0 * t110294 + 2.0 / 27.0 * t1901 * t28404 * t13938 + t1901 * t24793 * t14087 / 9.0 + 2.0 / 9.0 * t1901 * t24793 * t13943 + t446 * t729 * t762 * t1424 * t14012 / 3.0 - 4.0 / 3.0 * t1901 * t53942 * t28136 - 4.0 / 3.0 * t1901 * t51853 * t28129;
    (t110316,)
}
