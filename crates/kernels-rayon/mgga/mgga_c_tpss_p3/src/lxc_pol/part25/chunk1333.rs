//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1333/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1333(t6323: f64, t645: f64, t116: f64, t21907: f64, t13133: f64, t13554: f64, t13565: f64, t1760: f64, t1800: f64, t1845: f64, t18690: f64, t19620: f64, t20289: f64, t20343: f64, t20368: f64, t20379: f64, t2056: f64, t21180: f64, t21868: f64, t21908: f64, t3493: f64, t3499: f64, t3542: f64, t42710: f64, t44034: f64, t485: f64, t50656: f64, t5706: f64, t5809: f64, t5816: f64, t6103: f64, t626: f64, t6328: f64, t68838: f64, t69023: f64, t69026: f64, t71159: f64) -> (f64, f64, f64) {
    let t71184 = t645 * t6323;
    let t71212 = t116 * t21907;
    let t71259 = -12.0_f64 * t19620 * t18690 * t68838 - 2.0_f64 * t42710 * t1800 - 2.0_f64 * t50656 * t1800 - 2.0_f64 * t13565 * t5809 - 4.0_f64 * t20289 * t3542 - t1760 * t1845 * t44034 + 2.0_f64 * t5706 * t21868 - 2.0_f64 * t13565 * t5816 - 4.0_f64 * t69023 * t1800 - 4.0_f64 * t69026 * t1800 - 4.0_f64 * t21180 * t5809 - 4.0_f64 * t3493 * t20379 - 4.0_f64 * t6103 * t20368 - 2.0_f64 * t626 * t485 * t71159 - 4.0_f64 * t13133 * t6328 - 4.0_f64 * t13554 * t6328 - 4.0_f64 * t3493 * t20343 - 2.0_f64 * t2056 * t21908 - 2.0_f64 * t3499 * t21908;
    (t71184, t71212, t71259)
}
