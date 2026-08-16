//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1167/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1167(t1882: f64, t36114: f64, t1248: f64, t142999: f64, t114751: f64, t1501: f64, t10688: f64, t1091: f64, t1255: f64, t144060: f64, t144073: f64, t144087: f64, t1508: f64, t152631: f64, t153507: f64, t153558: f64, t153717: f64, t153720: f64, t1901: f64, t24890: f64, t2862: f64, t28719: f64, t29260: f64, t29369: f64, t296: f64, t33873: f64, t33994: f64, t36133: f64, t4162: f64, t4299: f64, t446: f64, t6260: f64, t6353: f64, t7124: f64, t7611: f64, t7686: f64, t835: f64, t840: f64, t871: f64) -> (f64, f64, f64) {
    let t154484 = t1882 * t36114;
    let t154492 = t142999 * t1248;
    let t154503 = t114751 * t1501;
    let t154532 = -t446 * t835 * t33994 * t1091 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t840 * t10688 * t36133 + 2.0_f64 / 3.0_f64 * t446 * t840 * t6353 * t29369 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t1255 * t33873 - 2.0_f64 / 9.0_f64 * t154484 + 4.0_f64 / 3.0_f64 * t446 * t296 * t153720 + 4.0_f64 / 3.0_f64 * t446 * t296 * t153717 - t446 * t296 * t154492 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t840 * t1508 * t28719 + 2.0_f64 / 9.0_f64 * t1901 * t24890 * t29260 - 2.0_f64 / 3.0_f64 * t446 * t296 * t154503 + 2.0_f64 / 3.0_f64 * t446 * t296 * t153507 + 2.0_f64 / 3.0_f64 * t446 * t840 * t871 * t6260 * t7124 + t144060 - 2.0_f64 * t446 * t296 * t152631 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t7686 * t4162 + 2.0_f64 / 3.0_f64 * t446 * t296 * t153558 + t446 * t840 * t871 * t7611 * t4299 / 3.0_f64 + t144073 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t144087;
    (t154492, t154503, t154532)
}
