//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1376/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1376(t1206: f64, t1844: f64, t6435: f64, t9895: f64, t1163: f64, t13452: f64, t13458: f64, t13470: f64, t13565: f64, t13965: f64, t1600: f64, t1760: f64, t1830: f64, t18547: f64, t18690: f64, t18898: f64, t19577: f64, t19579: f64, t19581: f64, t20288: f64, t20294: f64, t20357: f64, t21017: f64, t21863: f64, t21922: f64, t25469: f64, t4341: f64, t4631: f64, t4638: f64, t4675: f64, t485: f64, t51635: f64, t5706: f64, t5801: f64, t5820: f64, t5895: f64, t6309: f64, t6439: f64, t68989: f64, t71374: f64) -> f64 {
    let t72608 = t1206 * t1844;
    let t72633 = t6435 * t9895;
    let t72637 = -2.0_f64 * t13565 * t5820 - 2.0_f64 * t20288 * t1600 - 2.0_f64 * t6309 * t4341 + 12.0_f64 * t18547 * t20357 * t68989 - 3.0_f64 * t18547 * t18690 * t51635 + 6.0_f64 * t5706 * t21863 + 6.0_f64 * t1760 * t72608 * t21017 - 2.0_f64 * t18898 * t4675 - 2.0_f64 * t20294 * t4675 - 2.0_f64 * t5801 * t13470 - 6.0_f64 * t18547 * t25469 * t13965 - 2.0_f64 * t19577 * t6439 - 2.0_f64 * t71374 * t485 - 2.0_f64 * t21922 * t1163 - t13452 * t1830 - t4631 * t5895 - 2.0_f64 * t13458 * t1830 - 2.0_f64 * t4638 * t5895 + 4.0_f64 * t19579 * t72633 * t19581;
    t72637
}
