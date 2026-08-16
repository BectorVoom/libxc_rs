//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2433/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2433(t1020: f64, t14489: f64, t248: f64, t3101: f64, t3038: f64, t49650: f64, t1022: f64, t10403: f64, t10413: f64, t10480: f64, t10876: f64, t13975: f64, t13985: f64, t14143: f64, t14180: f64, t14211: f64, t14218: f64, t2244: f64, t2775: f64, t2776: f64, t3043: f64, t3071: f64, t3117: f64, t3132: f64, t360: f64, t42610: f64, t42613: f64, t42619: f64, t42622: f64, t42651: f64, t4582: f64) -> f64 {
    let t49757 = t1020 * t248 * t3101 * t14489;
    let t49771 = t49650 * t3038;
    let t49786 = -t42610 / 432.0_f64 - t42613 / 324.0_f64 - t42619 / 108.0_f64 - t42622 / 81.0_f64 + t49757 / 1536.0_f64 + 3.0_f64 / 512.0_f64 * t10480 * t4582 * t13975 * t13985 - 3.0_f64 / 512.0_f64 * t10876 * t4582 * t13975 * t3132 - t3117 * t14143 / 384.0_f64 + 5.0_f64 / 2304.0_f64 * t3117 * t14180 - t49771 * t3043 / 1024.0_f64 - t42651 / 216.0_f64 - t10403 * t3071 * t14211 * t2776 * t1022 / 384.0_f64 + t10413 * t3071 * t14218 * t360 * t2775 * t2244 / 768.0_f64;
    t49786
}
