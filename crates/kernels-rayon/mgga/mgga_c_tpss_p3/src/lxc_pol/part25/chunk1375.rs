//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1375/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1375(t118: f64, t13547: f64, t13974: f64, t1760: f64, t1834: f64, t18547: f64, t18690: f64, t18710: f64, t19579: f64, t20218: f64, t20219: f64, t20221: f64, t20357: f64, t20361: f64, t2056: f64, t21027: f64, t21576: f64, t21858: f64, t25232: f64, t3499: f64, t3538: f64, t4341: f64, t4525: f64, t51642: f64, t51664: f64, t5463: f64, t5706: f64, t5709: f64, t5801: f64, t5905: f64, t5909: f64, t6243: f64, t6245: f64, t626: f64, t6323: f64, t65533: f64, t67782: f64, t68823: f64, t71884: f64, t72425: f64, t72576: f64) -> f64 {
    let t72593 = -6.0_f64 * t18547 * t18690 * t51642 + 3.0_f64 * t1760 * t71884 * t5709 - 2.0_f64 * t1760 * t20218 * t4525 + 3.0_f64 * t1760 * t5909 * t68823 + 6.0_f64 * t5706 * t21858 - 6.0_f64 * t65533 * t20221 - 2.0_f64 * t6243 * t20361 + t1834 * t13974 + 3.0_f64 * t1760 * t18710 * t21027 + 6.0_f64 * t1760 * t67782 * t6245 + 2.0_f64 * t19579 * t20357 * t51664 - t118 * (t72425 + t72576) + 2.0_f64 * t6243 * t20219 - 4.0_f64 * t2056 * t21576 - 4.0_f64 * t3499 * t21576 - 4.0_f64 * t626 * t4341 * t6323 + t5905 * t5463 - 4.0_f64 * t25232 * t3538 - 2.0_f64 * t5801 * t13547;
    t72593
}
