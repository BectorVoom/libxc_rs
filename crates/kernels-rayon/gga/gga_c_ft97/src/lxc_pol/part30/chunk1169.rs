//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1169/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1169(t1882: f64, t36253: f64, t36195: f64, t681: f64, t89: f64, t36264: f64, t36172: f64, t36199: f64, t112952: f64, t1255: f64, t143989: f64, t152648: f64, t152856: f64, t15369: f64, t1901: f64, t24886: f64, t2862: f64, t29083: f64, t29203: f64, t29208: f64, t29245: f64, t29399: f64, t296: f64, t319: f64, t33953: f64, t4167: f64, t446: f64, t53797: f64, t54032: f64, t6353: f64, t840: f64) -> (f64, f64, f64, f64) {
    let t154591 = t1882 * t36253;
    let t154602 = t89 * t681 * t36195;
    let t154604 = t1882 * t36264;
    let t154626 = t1882 * t36172;
    let t154631 = t1882 * t36199;
    let t154656 = 4.0_f64 / 9.0_f64 * t53797 * t112952 * t29203 - 4.0_f64 / 27.0_f64 * t54032 * t112952 * t29208 + t154626 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t296 * t152648 + t154631 / 9.0_f64 - t446 * t840 * t1255 * t33953 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t24886 * t29083 + 2.0_f64 / 3.0_f64 * t446 * t840 * t6353 * t29245 + 2.0_f64 / 3.0_f64 * t446 * t840 * t6353 * t29399 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t319 * t152856 - 2.0_f64 / 3.0_f64 * t1901 * t15369 * t143989 * t4167;
    (t154591, t154602, t154604, t154656)
}
