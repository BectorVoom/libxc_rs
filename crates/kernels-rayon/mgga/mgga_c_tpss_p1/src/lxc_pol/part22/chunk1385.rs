//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1385/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1385(t1600: f64, t1760: f64, t18295: f64, t1845: f64, t18544: f64, t18628: f64, t18687: f64, t18694: f64, t18710: f64, t18898: f64, t18903: f64, t19604: f64, t20289: f64, t20358: f64, t20368: f64, t20407: f64, t2056: f64, t2065: f64, t2105: f64, t3499: f64, t3537: f64, t3542: f64, t41867: f64, t5706: f64, t5895: f64, t5909: f64, t6103: f64, t6243: f64, t626: f64, t63710: f64, t6399: f64, t6413: f64, t6436: f64, t646: f64, t65501: f64, t67541: f64, t9909: f64) -> f64 {
    let t67633 = 3.0_f64 * t1760 * t5909 * t65501 - 4.0_f64 * t2056 * t20368 - 4.0_f64 * t3499 * t20368 - 4.0_f64 * t626 * t5895 * t3537 + 6.0_f64 * t6243 * t18687 + 6.0_f64 * t5706 * t20407 - t1760 * t1845 * t41867 + 3.0_f64 * t18544 * t6413 + 6.0_f64 * t1760 * t18710 * t19604 - t1760 * t6436 * t9909 - 4.0_f64 * t67541 * t646 - 4.0_f64 * t20289 * t2065 - 2.0_f64 * t6243 * t18694 - 2.0_f64 * t626 * t6399 * t2105 - 4.0_f64 * t18898 * t3542 - 2.0_f64 * t18903 * t1600 + 4.0_f64 * t63710 * t20358 + 2.0_f64 * t1760 * t6436 * t18295 - 2.0_f64 * t6103 * t18628;
    t67633
}
