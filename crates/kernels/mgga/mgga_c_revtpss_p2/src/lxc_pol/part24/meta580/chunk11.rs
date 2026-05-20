//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1804/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1804<F: Float>(t6702: F, t6744: F, t1210: F, t1211: F, t1274: F, t1277: F, t1774: F, t1775: F, t17973: F, t17974: F, t20700: F, t24515: F, t24525: F, t24899: F, t25015: F, t25019: F, t3567: F, t3737: F, t45438: F, t45552: F, t5220: F, t5225: F, t5251: F, t56332: F, t56393: F, t6564: F, t6573: F, t6574: F, t6697: F, t6703: F, t6745: F, t72767: F, t84315: F, t91037: F, t91473: F, t91513: F, t91544: F, t91576: F, t91609: F, t91642: F, t91671: F, t91706: F) -> F {
    let t91727 = t6702 * t6702;
    let t91731 = t6744 * t6744;
    let t91748 = F::cast_from(0.26341796731742046395e1_f64) * t1210 * t1277 * t25015 * t1774 - F::cast_from(0.15805078039045227836e2_f64) * t17973 * t17974 * t24899 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1277 * (t91473 + t91513 + t91544 + t91576 + t91609 + t91642 + t91671 + t91706) - F::cast_from(0.26341796731742046395e1_f64) * t84315 * t1775 + F::cast_from(0.79025390195226139183e1_f64) * t72767 * t6574 + F::cast_from(0.79025390195226139183e1_f64) * t5220 * t24515 - F::cast_from(0.79025390195226139183e1_f64) * t3567 * t1277 * t6573 * t6744 - F::cast_from(0.15805078039045227836e2_f64) * t5225 * t24525 - F::cast_from(0.15805078039045227836e2_f64) * t56332 * t25019 + F::cast_from(0.15805078039045227836e2_f64) * t1274 * t45552 * t91727 + F::cast_from(0.39512695097613069591e1_f64) * t1274 * t3737 * t91731 + F::cast_from(0.79025390195226139183e1_f64) * t5251 * t24515 - F::cast_from(0.39512695097613069592e1_f64) * t20700 * t6745 + F::cast_from(0.39512695097613069592e1_f64) * t6564 * t6697 + F::cast_from(0.79025390195226139183e1_f64) * t20700 * t6703 + F::cast_from(0.15805078039045227836e2_f64) * t45438 * t1211 * t91037 - F::cast_from(0.15805078039045227836e2_f64) * t56393 * t25019;
    t91748
}
