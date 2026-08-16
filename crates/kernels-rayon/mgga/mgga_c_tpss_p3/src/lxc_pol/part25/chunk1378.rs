//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1378/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1378(t1163: f64, t13463: f64, t13478: f64, t13546: f64, t1830: f64, t18547: f64, t18690: f64, t18898: f64, t19305: f64, t19308: f64, t19620: f64, t20294: f64, t20358: f64, t20386: f64, t20396: f64, t20407: f64, t21236: f64, t21786: f64, t21871: f64, t4641: f64, t485: f64, t51631: f64, t5706: f64, t5801: f64, t5816: f64, t6103: f64, t6243: f64, t626: f64, t6318: f64, t6324: f64, t68950: f64, t68967: f64, t71549: f64) -> f64 {
    let t72721 = -2.0_f64 * t21236 * t5816 - 4.0_f64 * t19305 * t6318 - 4.0_f64 * t19308 * t6318 - 4.0_f64 * t6103 * t20396 - 2.0_f64 * t626 * t1830 * t13546 - 4.0_f64 * t18898 * t4641 - 4.0_f64 * t20294 * t4641 - 4.0_f64 * t5801 * t13478 - 2.0_f64 * t5801 * t13463 - 4.0_f64 * t19305 * t6324 - 4.0_f64 * t19308 * t6324 - 4.0_f64 * t6103 * t20386 - 6.0_f64 * t19620 * t18690 * t51631 - t5706 * t21871 - 6.0_f64 * t18547 * t18690 * t68950 + 4.0_f64 * t68967 * t20358 + 6.0_f64 * t6243 * t20407 - t71549 * t485 - t21786 * t1163;
    t72721
}
