//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1379/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1379<F: Float>(t26925: F, t8392: F, t26929: F, t27229: F, t1882: F, t26940: F, t358: F, t6718: F, t104388: F, t105781: F, t12956: F, t13088: F, t13220: F, t144: F, t1651: F, t167: F, t1901: F, t1969: F, t2185: F, t2212: F, t2223: F, t23429: F, t3281: F, t3578: F, t446: F, t569: F, t574: F, t5916: F, t5935: F, t6725: F, t9144: F, t9432: F, t95890: F, t95898: F) -> (F,) {
    let t107111 = 4.0 / 9.0 * t8392 * t26925;
    let t107113 = 4.0 / 9.0 * t8392 * t26929;
    let t107115 = 2.0 / 27.0 * t8392 * t27229;
    let t107117 = 2.0 / 9.0 * t1882 * t26940;
    let t107141 = t6718 * t358;
    let t107152 = -2.0 * t446 * t9432 * t167 * t105781 + t107111 + t107113 + t107115 - t107117 - 2.0 / 3.0 * t446 * t2185 * t5935 * t13088 + 2.0 / 9.0 * t3281 * t1969 * t167 * t5916 + 2.0 / 3.0 * t446 * t574 * t5935 * t12956 + 2.0 / 3.0 * t446 * t574 * t3578 * t23429 + 4.0 / 3.0 * t446 * t144 * t104388 - t446 * t569 * t6725 * t1651 / 9.0 - 2.0 / 9.0 * t1901 * t9144 * t107141 * t2223 - 4.0 / 9.0 * t1901 * t13220 * t107141 * t2212 - 2.0 / 81.0 * t95890 - 2.0 / 27.0 * t95898;
    (t107152,)
}
