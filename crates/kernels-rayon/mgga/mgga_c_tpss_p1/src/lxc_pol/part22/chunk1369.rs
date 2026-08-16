//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1369/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1369(t118: f64, t12664: f64, t1270: f64, t12836: f64, t12841: f64, t1338: f64, t13554: f64, t1760: f64, t1799: f64, t18547: f64, t18613: f64, t18690: f64, t18691: f64, t18707: f64, t18896: f64, t19305: f64, t19308: f64, t19579: f64, t19581: f64, t19620: f64, t20134: f64, t20346: f64, t2062: f64, t26207: f64, t3493: f64, t42962: f64, t4478: f64, t509: f64, t5816: f64, t6103: f64, t61801: f64, t626: f64, t6399: f64, t65056: f64, t65060: f64, t65533: f64, t66217: f64, t66764: f64, t66912: f64, t66998: f64, t67057: f64, t67109: f64, t67211: f64, t7383: f64) -> f64 {
    let t67223 = -6.0_f64 * t18547 * t18690 * t42962 + 4.0_f64 * t19579 * t66217 * t19581 - 6.0_f64 * t65533 * t18691 + 12.0_f64 * t65056 * t20134 + 12.0_f64 * t19620 * t26207 * t4478 - t118 * (t66764 + t66912) - 2.0_f64 * t2062 * t6399 + 12.0_f64 * t19620 * t7383 * t12836 + 6.0_f64 * t19620 * t7383 * t12841 - 3.0_f64 * t18547 * t18690 * t65060 - 6.0_f64 * t61801 * t20346 - 4.0_f64 * t19305 * t5816 - 4.0_f64 * t19308 * t5816 - 4.0_f64 * t6103 * t18707 - 4.0_f64 * t13554 * t5816 - 2.0_f64 * t3493 * t18613 + t1760 * t509 * (t66998 + t67057 + t67109 + t67211) * t1270 - 2.0_f64 * t626 * t12664 * t1799 - 2.0_f64 * t626 * t18896 * t1338;
    t67223
}
