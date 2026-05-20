//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3088/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3088<F: Float>(t17331: F, t487: F, t1204: F, t5412: F, t1811: F, t3552: F, t1269: F, t17288: F, t3584: F, t5245: F, t1210: F, t1211: F, t1215: F, t12607: F, t12633: F, t12651: F, t12658: F, t12696: F, t1274: F, t1277: F, t1295: F, t13183: F, t17999: F, t18047: F, t18062: F, t18084: F, t18087: F, t18103: F, t1828: F, t1829: F, t3556: F, t3567: F, t3572: F, t3585: F, t3791: F, t45430: F, t45487: F, t45552: F, t5220: F, t5225: F, t5231: F, t5251: F, t5423: F, t5497: F) -> (F, F) {
    let t56486 = t17331 * t487;
    let t56503 = t1204 * t5412;
    let t56508 = t3552 * t1811;
    let t56519 = t17288 * t1269;
    let t56530 = t5245 * t3584;
    let t56534 = -F::cast_from(0.19756347548806534796e1_f64) * t56486 * t1295 + F::cast_from(0.39512695097613069591e1_f64) * t5225 * t12696 + F::cast_from(0.15805078039045227836e2_f64) * t1274 * t45552 * t1828 * t13183 + F::cast_from(0.19756347548806534796e1_f64) * t1210 * t1277 * t5497 * t3584 + F::cast_from(0.19756347548806534796e1_f64) * t3556 * t18084 - F::cast_from(0.39512695097613069591e1_f64) * t3572 * t18047 - F::cast_from(0.39512695097613069591e1_f64) * t56503 * t1295 + F::cast_from(0.19756347548806534796e1_f64) * t5220 * t12651 - F::cast_from(0.19756347548806534796e1_f64) * t56508 * t1295 + F::cast_from(0.39512695097613069591e1_f64) * t45430 * t5231 - F::cast_from(0.39512695097613069591e1_f64) * t12633 * t18103 + F::cast_from(0.19756347548806534796e1_f64) * t5251 * t12607 - F::cast_from(0.19756347548806534796e1_f64) * t18087 * t3791 - F::cast_from(0.39512695097613069591e1_f64) * t56519 * t1215 - F::cast_from(0.65854491829355115987e0_f64) * t45487 * t1829 - F::cast_from(0.19756347548806534796e1_f64) * t18062 * t3585 + F::cast_from(0.19756347548806534796e1_f64) * t12658 * t5423 + F::cast_from(0.19756347548806534796e1_f64) * t3572 * t17999 + F::cast_from(0.39512695097613069591e1_f64) * t3567 * t1211 * t56530;
    (t56530, t56534)
}
