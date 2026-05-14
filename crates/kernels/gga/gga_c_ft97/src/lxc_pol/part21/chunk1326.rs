//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1326/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1326<F: Float>(t1882: F, t30520: F, t1017: F, t106253: F, t106698: F, t11593: F, t119348: F, t119470: F, t119501: F, t119584: F, t12968: F, t13140: F, t13208: F, t144: F, t16150: F, t16950: F, t16955: F, t17099: F, t17388: F, t1901: F, t23455: F, t26955: F, t27191: F, t27220: F, t30281: F, t3478: F, t3483: F, t3578: F, t41269: F, t446: F, t49583: F, t50240: F, t558: F, t574: F, t5942: F, t605: F, t63755: F, t6699: F) -> (F,) {
    let t121286 = t1882 * t30520;
    let t121341 = 2.0 / 9.0 * t121286 + 2.0 / 3.0 * t446 * t574 * t605 * t27191 * t1017 - 2.0 / 3.0 * t1901 * t12968 * t5942 * t17099 - 2.0 / 3.0 * t1901 * t13140 * t23455 * t17388 + 2.0 / 3.0 * t446 * t574 * t3578 * t26955 + 4.0 / 3.0 * t446 * t144 * t119470 + 8.0 / 3.0 * t1901 * t63755 * t6699 * t3478 + 4.0 * t1901 * t106698 * t6699 * t3483 - 8.0 / 9.0 * t11593 * t13208 * t119584 - t446 * t574 * t30281 * t558 / 3.0 + 4.0 / 3.0 * t446 * t144 * t119501 + 4.0 / 3.0 * t446 * t144 * t119348 - 2.0 / 27.0 * t1901 * t41269 * t27220 * t16950 - 4.0 / 27.0 * t1901 * t50240 * t27220 * t16955 + 10.0 / 81.0 * t1901 * t49583 * t106253 * t16150;
    (t121341,)
}
