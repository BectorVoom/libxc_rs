//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3032/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3032<F: Float>(t4746: F, t4995: F, t1087: F, t1089: F, t12066: F, t12079: F, t12094: F, t12122: F, t12128: F, t12168: F, t16381: F, t1647: F, t16540: F, t16578: F, t19608: F, t3259: F, t3304: F, t3309: F, t43357: F, t43443: F, t43453: F, t43520: F, t43524: F, t43562: F, t43598: F, t4866: F, t4893: F, t4977: F, t4981: F, t53792: F, t54276: F) -> F {
    let t55732 = t4746 * t4995;
    let t55746 = -F::cast_from(0.19756347548806534796e1_f64) * t19608 * t12094 + F::cast_from(0.39512695097613069591e1_f64) * t43443 * t16578 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t53792 * t3304 - F::cast_from(0.39512695097613069591e1_f64) * t43520 * t54276 * t12168 + F::cast_from(0.39512695097613069591e1_f64) * t43524 * t54276 * t12079 + F::cast_from(0.19756347548806534796e1_f64) * t43453 * t16540 - F::cast_from(0.19756347548806534796e1_f64) * t43357 * t4977 + F::cast_from(0.39512695097613069591e1_f64) * t16381 * t3309 + F::cast_from(0.19756347548806534796e1_f64) * t55732 * t12128 + F::cast_from(0.19756347548806534796e1_f64) * t1087 * t3259 * t4866 * t1089 + F::cast_from(0.39512695097613069591e1_f64) * t43598 * t16578 + F::cast_from(0.65854491829355115987e0_f64) * t1647 * t12066 + F::cast_from(0.13170898365871023197e1_f64) * t4981 * t4893 * t43562;
    t55746
}
