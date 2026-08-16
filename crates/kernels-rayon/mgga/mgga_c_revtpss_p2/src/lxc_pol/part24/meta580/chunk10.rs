//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1803/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1803(t12717: f64, t12751: f64, t1280: f64, t12987: f64, t13142: f64, t13143: f64, t16695: f64, t1774: f64, t1822: f64, t21442: f64, t24931: f64, t24964: f64, t3670: f64, t45608: f64, t45610: f64, t45619: f64, t45620: f64, t45833: f64, t45834: f64, t490: f64, t5326: f64, t5457: f64, t59948: f64, t6587: f64, t84645: f64, t84859: f64, t89883: f64, t90926: f64, t91272: f64, t91536: f64, t91610: f64) -> f64 {
    let t91706 = -0.15805078039045227836e2_f64 * t12751 * t16695 * t84645 * t1774 + 0.15805078039045227836e2_f64 * t12717 * t21442 * t5457 * t6587 - 0.23707617058567841754e2_f64 * t12987 * t1280 * t91272 - 0.23707617058567841754e2_f64 * t45608 * t91536 * t45610 + 0.15805078039045227836e2_f64 * t45619 * t91536 * t45620 + 0.15805078039045227836e2_f64 * t59948 * t24931 + 0.65854491829355115987e0_f64 * t89883 * t490 + 0.26341796731742046395e1_f64 * t84859 * t1822 + 0.52683593463484092788e1_f64 * t3670 * t1280 * t90926 - 0.26341796731742046395e1_f64 * t5326 * t24964 - 0.65854491829355115987e0_f64 * t45833 * t91536 * t45834 - 0.15805078039045227836e2_f64 * t13142 * t91610 * t13143;
    t91706
}
