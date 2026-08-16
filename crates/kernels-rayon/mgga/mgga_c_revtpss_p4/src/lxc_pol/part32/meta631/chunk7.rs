//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2050/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2050(t105892: f64, t109199: f64, t1502: f64, t18220: f64, t18232: f64, t1843: f64, t1911: f64, t2014: f64, t2052: f64, t2089: f64, t21658: f64, t22279: f64, t25082: f64, t26405: f64, t28167: f64, t28176: f64, t28286: f64, t28586: f64, t28652: f64, t28686: f64, t28707: f64, t28718: f64, t28929: f64, t28938: f64, t30314: f64, t4246: f64, t5517: f64, t5787: f64, t5884: f64, t6765: f64, t7315: f64, t7357: f64, t7359: f64, t7474: f64, t7898: f64, t7969: f64, t8065: f64, t8075: f64, t86753: f64, t9069: f64, t98450: f64) -> f64 {
    let t111301 = -2.0_f64 * t4246 * t8065 - 2.0_f64 * t1502 * t28586 - t2014 * t30314 * t7315 - 2.0_f64 * t7359 * t18232 - 2.0_f64 * t18220 * t2089 - 2.0_f64 * t5884 * t7474 + 12.0_f64 * t105892 * t28929 - 6.0_f64 * t28167 * t26405 * t86753 + 12.0_f64 * t28167 * t9069 * t22279 + 6.0_f64 * t2014 * t28938 * t28176 - 2.0_f64 * t28652 * t1843 - 2.0_f64 * t7969 * t5517 - t7357 * t6765 - t2052 * t21658 + 2.0_f64 * t8075 * t5787 - 6.0_f64 * t98450 * t28718 + 12.0_f64 * t25082 * t28286 * t109199 + 2.0_f64 * t28686 * t1911 - 2.0_f64 * t7898 * t28707;
    t111301
}
