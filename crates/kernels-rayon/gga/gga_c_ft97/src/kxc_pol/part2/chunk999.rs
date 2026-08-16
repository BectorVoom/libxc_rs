//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 999/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk999(t1882: f64, t4188: f64, t1255: f64, t2409: f64, t835: f64, t10735: f64, t10745: f64, t10749: f64, t10750: f64, t10752: f64, t15455: f64, t15463: f64, t15467: f64, t15471: f64, t15474: f64, t15479: f64, t15482: f64, t15487: f64, t1901: f64, t446: f64) -> f64 {
    let t15491 = 2.0_f64 / 27.0_f64 * t1882 * t4188;
    let t15493 = t835 * t1255 * t2409;
    let t15496 = 2.0_f64 / 9.0_f64 * t1901 * t15455 - 8.0_f64 / 27.0_f64 * t10735 - t10745 / 9.0_f64 - t10749 - 4.0_f64 / 3.0_f64 * t1901 * t15463 - t15467 + t10750 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t10752 + t15471 - 2.0_f64 / 3.0_f64 * t446 * t15474 - 2.0_f64 * t446 * t15479 - 2.0_f64 * t446 * t15482 - 2.0_f64 / 3.0_f64 * t446 * t15487 + t15491 + 2.0_f64 / 9.0_f64 * t446 * t15493;
    t15496
}
