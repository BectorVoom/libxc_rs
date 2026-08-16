//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3032/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3032(t4746: f64, t4995: f64, t1087: f64, t1089: f64, t12066: f64, t12079: f64, t12094: f64, t12122: f64, t12128: f64, t12168: f64, t16381: f64, t1647: f64, t16540: f64, t16578: f64, t19608: f64, t3259: f64, t3304: f64, t3309: f64, t43357: f64, t43443: f64, t43453: f64, t43520: f64, t43524: f64, t43562: f64, t43598: f64, t4866: f64, t4893: f64, t4977: f64, t4981: f64, t53792: f64, t54276: f64) -> f64 {
    let t55732 = t4746 * t4995;
    let t55746 = -0.19756347548806534796e1_f64 * t19608 * t12094 + 0.39512695097613069591e1_f64 * t43443 * t16578 - 0.39512695097613069591e1_f64 * t12122 * t53792 * t3304 - 0.39512695097613069591e1_f64 * t43520 * t54276 * t12168 + 0.39512695097613069591e1_f64 * t43524 * t54276 * t12079 + 0.19756347548806534796e1_f64 * t43453 * t16540 - 0.19756347548806534796e1_f64 * t43357 * t4977 + 0.39512695097613069591e1_f64 * t16381 * t3309 + 0.19756347548806534796e1_f64 * t55732 * t12128 + 0.19756347548806534796e1_f64 * t1087 * t3259 * t4866 * t1089 + 0.39512695097613069591e1_f64 * t43598 * t16578 + 0.65854491829355115987e0_f64 * t1647 * t12066 + 0.13170898365871023197e1_f64 * t4981 * t4893 * t43562;
    t55746
}
