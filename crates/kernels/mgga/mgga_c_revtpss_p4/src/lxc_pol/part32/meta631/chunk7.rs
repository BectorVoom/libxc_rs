//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2050/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2050<F: Float>(t105892: F, t109199: F, t1502: F, t18220: F, t18232: F, t1843: F, t1911: F, t2014: F, t2052: F, t2089: F, t21658: F, t22279: F, t25082: F, t26405: F, t28167: F, t28176: F, t28286: F, t28586: F, t28652: F, t28686: F, t28707: F, t28718: F, t28929: F, t28938: F, t30314: F, t4246: F, t5517: F, t5787: F, t5884: F, t6765: F, t7315: F, t7357: F, t7359: F, t7474: F, t7898: F, t7969: F, t8065: F, t8075: F, t86753: F, t9069: F, t98450: F) -> F {
    let t111301 = -F::cast_from(2.0_f64) * t4246 * t8065 - F::cast_from(2.0_f64) * t1502 * t28586 - t2014 * t30314 * t7315 - F::cast_from(2.0_f64) * t7359 * t18232 - F::cast_from(2.0_f64) * t18220 * t2089 - F::cast_from(2.0_f64) * t5884 * t7474 + F::cast_from(12.0_f64) * t105892 * t28929 - F::cast_from(6.0_f64) * t28167 * t26405 * t86753 + F::cast_from(12.0_f64) * t28167 * t9069 * t22279 + F::cast_from(6.0_f64) * t2014 * t28938 * t28176 - F::cast_from(2.0_f64) * t28652 * t1843 - F::cast_from(2.0_f64) * t7969 * t5517 - t7357 * t6765 - t2052 * t21658 + F::cast_from(2.0_f64) * t8075 * t5787 - F::cast_from(6.0_f64) * t98450 * t28718 + F::cast_from(12.0_f64) * t25082 * t28286 * t109199 + F::cast_from(2.0_f64) * t28686 * t1911 - F::cast_from(2.0_f64) * t7898 * t28707;
    t111301
}
