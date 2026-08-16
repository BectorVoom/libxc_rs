//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1803/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1803<F: Float>(t12717: F, t12751: F, t1280: F, t12987: F, t13142: F, t13143: F, t16695: F, t1774: F, t1822: F, t21442: F, t24931: F, t24964: F, t3670: F, t45608: F, t45610: F, t45619: F, t45620: F, t45833: F, t45834: F, t490: F, t5326: F, t5457: F, t59948: F, t6587: F, t84645: F, t84859: F, t89883: F, t90926: F, t91272: F, t91536: F, t91610: F) -> F {
    let t91706 = -F::cast_from(0.15805078039045227836e2_f64) * t12751 * t16695 * t84645 * t1774 + F::cast_from(0.15805078039045227836e2_f64) * t12717 * t21442 * t5457 * t6587 - F::cast_from(0.23707617058567841754e2_f64) * t12987 * t1280 * t91272 - F::cast_from(0.23707617058567841754e2_f64) * t45608 * t91536 * t45610 + F::cast_from(0.15805078039045227836e2_f64) * t45619 * t91536 * t45620 + F::cast_from(0.15805078039045227836e2_f64) * t59948 * t24931 + F::cast_from(0.65854491829355115987e0_f64) * t89883 * t490 + F::cast_from(0.26341796731742046395e1_f64) * t84859 * t1822 + F::cast_from(0.52683593463484092788e1_f64) * t3670 * t1280 * t90926 - F::cast_from(0.26341796731742046395e1_f64) * t5326 * t24964 - F::cast_from(0.65854491829355115987e0_f64) * t45833 * t91536 * t45834 - F::cast_from(0.15805078039045227836e2_f64) * t13142 * t91610 * t13143;
    t91706
}
