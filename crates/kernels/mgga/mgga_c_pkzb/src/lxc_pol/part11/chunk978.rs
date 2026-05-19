//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 978/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk978<F: Float>(t1034: F, t1044: F, t10573: F, t10577: F, t10621: F, t10675: F, t10692: F, t10696: F, t164: F, t167: F, t1717: F, t1721: F, t183: F, t2682: F, t2693: F, t3441: F, t3460: F, t5389: F, t5391: F, t588: F) -> F {
    let t10727 = -F::cast_from(0.39512695097613069591e1_f64) * t5389 * t10692 * t5391 + F::cast_from(0.39512695097613069591e1_f64) * t1717 * t10696 * t1721 + F::cast_from(0.39512695097613069591e1_f64) * t2682 * t10577 + F::cast_from(0.39512695097613069591e1_f64) * t1717 * t10692 * t1721 - F::cast_from(0.19756347548806534796e1_f64) * t588 * t3460 * t1034 * t164 - F::cast_from(0.19756347548806534796e1_f64) * t588 * t1044 * t3441 * t164 - F::cast_from(0.19756347548806534796e1_f64) * t588 * t10696 * t164 - F::cast_from(0.65854491829355115987e0_f64) * t588 * t183 * t10621 * t164 - F::cast_from(0.19756347548806534796e1_f64) * t2693 * t10573 - F::cast_from(0.65854491829355115987e0_f64) * t588 * t10692 * t164 + F::cast_from(0.65854491829355115987e0_f64) * t167 * t10675;
    t10727
}
