//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1026/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1026<F: Float>(t1882: F, t36114: F, t1248: F, t142999: F, t114751: F, t1501: F, t10688: F, t1091: F, t1255: F, t144060: F, t144073: F, t144087: F, t1508: F, t152631: F, t153507: F, t153558: F, t153717: F, t153720: F, t1901: F, t24890: F, t2862: F, t28719: F, t29260: F, t29369: F, t296: F, t33873: F, t33994: F, t36133: F, t4162: F, t4299: F, t446: F, t6260: F, t6353: F, t7124: F, t7611: F, t7686: F, t835: F, t840: F, t871: F) -> (F, F, F) {
    let t154484 = t1882 * t36114;
    let t154492 = t142999 * t1248;
    let t154503 = t114751 * t1501;
    let t154532 = -t446 * t835 * t33994 * t1091 / 9.0 - 2.0 / 3.0 * t446 * t840 * t10688 * t36133 + 2.0 / 3.0 * t446 * t840 * t6353 * t29369 + 2.0 / 3.0 * t446 * t2862 * t1255 * t33873 - 2.0 / 9.0 * t154484 + 4.0 / 3.0 * t446 * t296 * t153720 + 4.0 / 3.0 * t446 * t296 * t153717 - t446 * t296 * t154492 / 3.0 - 2.0 / 3.0 * t446 * t840 * t1508 * t28719 + 2.0 / 9.0 * t1901 * t24890 * t29260 - 2.0 / 3.0 * t446 * t296 * t154503 + 2.0 / 3.0 * t446 * t296 * t153507 + 2.0 / 3.0 * t446 * t840 * t871 * t6260 * t7124 + t144060 - 2.0 * t446 * t296 * t152631 + 2.0 / 3.0 * t446 * t2862 * t7686 * t4162 + 2.0 / 3.0 * t446 * t296 * t153558 + t446 * t840 * t871 * t7611 * t4299 / 3.0 + t144073 / 9.0 - 2.0 / 9.0 * t144087;
    (t154492, t154503, t154532)
}
