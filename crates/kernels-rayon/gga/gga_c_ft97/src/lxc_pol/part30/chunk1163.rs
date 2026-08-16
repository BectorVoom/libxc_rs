//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1163/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1163(t1882: f64, t36208: f64, t28859: f64, t6386: f64, t36183: f64, t8392: f64, t10703: f64, t1212: f64, t143789: f64, t143823: f64, t15195: f64, t152844: f64, t15299: f64, t153509: f64, t15369: f64, t154083: f64, t154090: f64, t154221: f64, t154225: f64, t154235: f64, t1901: f64, t24898: f64, t2749: f64, t28496: f64, t29071: f64, t29185: f64, t29293: f64, t296: f64, t34053: f64, t34198: f64, t36112: f64, t36186: f64, t446: f64, t6273: f64, t6287: f64, t684: f64, t840: f64, t871: f64, t99238: f64) -> (f64, f64) {
    let t154240 = t1882 * t36208;
    let t154242 = t28859 * t6386;
    let t154256 = t8392 * t36183;
    let t154268 = -t446 * t296 * t154083 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t15195 * t34198 - 2.0_f64 / 27.0_f64 * t154090 + 2.0_f64 / 3.0_f64 * t446 * t840 * t28859 * t6287 - 4.0_f64 / 9.0_f64 * t143789 - t446 * t296 * t154221 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t154225 - 4.0_f64 * t1901 * t29071 * t6273 * t28496 - 4.0_f64 / 3.0_f64 * t1901 * t15369 * t24898 * t29293 - 2.0_f64 / 9.0_f64 * t154235 + 4.0_f64 / 3.0_f64 * t446 * t296 * t153509 + 2.0_f64 / 9.0_f64 * t154240 - 2.0_f64 / 3.0_f64 * t446 * t296 * t154242 - 2.0_f64 / 9.0_f64 * t143823 + t446 * t840 * t871 * t34053 * t1212 / 3.0_f64 + t446 * t840 * t2749 * t36186 / 3.0_f64 + 2.0_f64 / 27.0_f64 * t154256 - 2.0_f64 / 9.0_f64 * t1901 * t10703 * t36112 * t684 - 2.0_f64 / 9.0_f64 * t1901 * t15299 * t152844 - 2.0_f64 / 9.0_f64 * t1901 * t99238 * t29185;
    (t154242, t154268)
}
