//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3209/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3209<F: Float>(t12633: F, t1274: F, t1775: F, t17973: F, t17995: F, t18087: F, t1828: F, t20710: F, t20722: F, t20741: F, t20756: F, t20760: F, t21390: F, t21394: F, t21617: F, t21618: F, t21621: F, t225: F, t24515: F, t24892: F, t24900: F, t25016: F, t3556: F, t3572: F, t3732: F, t3736: F, t3737: F, t460: F, t494: F, t5220: F, t5246: F, t5251: F, t5417: F, t5422: F, t5428: F, t5429: F, t6573: F, t6745: F, t68658: F, t72808: F, t73051: F, t73055: F, t84203: F) -> F {
    let t84241 = -F::cast_from(0.65854491829355115987e0_f64) * t3732 * t25016 + F::cast_from(0.19756347548806534796e1_f64) * t3572 * t24515 - F::cast_from(0.19756347548806534796e1_f64) * t18087 * t6745 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t3737 * t1828 * t21617 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t84203 * t225 * t494 - F::cast_from(0.39512695097613069591e1_f64) * t68658 * t1775 + F::cast_from(0.79025390195226139182e1_f64) * t20756 * t5429 - F::cast_from(0.39512695097613069592e1_f64) * t17973 * t73051 * t5422 + F::cast_from(0.19756347548806534796e1_f64) * t3556 * t24900 + F::cast_from(0.79025390195226139182e1_f64) * t17973 * t3736 * t6573 * t5428 - F::cast_from(0.19756347548806534796e1_f64) * t21621 * t5246 - F::cast_from(0.39512695097613069591e1_f64) * t5220 * t20741 + F::cast_from(0.19756347548806534796e1_f64) * t5251 * t20710 + F::cast_from(0.79025390195226139182e1_f64) * t17995 * t20722 + F::cast_from(0.39512695097613069592e1_f64) * t5417 * t20760 - F::cast_from(0.79025390195226139182e1_f64) * t72808 * t21390 + F::cast_from(0.39512695097613069591e1_f64) * t12633 * t24892 - F::cast_from(0.19756347548806534796e1_f64) * t5417 * t21618 - F::cast_from(0.19756347548806534796e1_f64) * t73055 * t1775 - F::cast_from(0.39512695097613069591e1_f64) * t21394 * t5246;
    t84241
}
