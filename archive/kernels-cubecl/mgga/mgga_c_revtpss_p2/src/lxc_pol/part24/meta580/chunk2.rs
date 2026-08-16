//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1795/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1795<F: Float>(t1210: F, t1211: F, t12628: F, t1274: F, t1775: F, t17995: F, t18059: F, t1813: F, t1828: F, t1829: F, t21621: F, t225: F, t24698: F, t24892: F, t24906: F, t25015: F, t25016: F, t3567: F, t3737: F, t460: F, t494: F, t5225: F, t6580: F, t6587: F, t6702: F, t82150: F, t82204: F, t82217: F, t82238: F, t84967: F, t89960: F, t90926: F, t91272: F, t91403: F) -> F {
    let t91440 = -F::cast_from(0.15805078039045227836e2_f64) * t18059 * t24906 - F::cast_from(0.15805078039045227836e2_f64) * t17995 * t24906 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1211 * t89960 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t91403 * t225 * t494 - F::cast_from(0.79025390195226139183e1_f64) * t82217 * t1829 + F::cast_from(0.52683593463484092788e1_f64) * t1274 * t3737 * t25015 * t1828 + F::cast_from(0.79025390195226139183e1_f64) * t21621 * t6580 - F::cast_from(0.79025390195226139183e1_f64) * t82150 * t1775 - F::cast_from(0.79025390195226139183e1_f64) * t82238 * t1829 - F::cast_from(0.26341796731742046395e1_f64) * t84967 * t1775 - F::cast_from(0.79025390195226139183e1_f64) * t1210 * t3737 * t6587 * t6702 - F::cast_from(0.26341796731742046395e1_f64) * t5225 * t25016 + F::cast_from(0.26341796731742046395e1_f64) * t24698 * t1813 + F::cast_from(0.15805078039045227836e2_f64) * t17995 * t24892 - F::cast_from(0.23707617058567841754e2_f64) * t12628 * t1211 * t91272 + F::cast_from(0.52683593463484092788e1_f64) * t3567 * t1211 * t90926 - F::cast_from(0.79025390195226139183e1_f64) * t82204 * t1775;
    t91440
}
