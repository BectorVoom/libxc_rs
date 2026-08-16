//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1679/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1679(t1243: f64, t42859: f64, t460: f64, t43351: f64, t471: f64, t1234: f64, t1269: f64, t12732: f64, t1280: f64, t1285: f64, t1287: f64, t12966: f64, t12987: f64, t13127: f64, t13129: f64, t13156: f64, t3552: f64, t3787: f64, t44639: f64, t44778: f64, t44843: f64, t44845: f64, t44878: f64, t45329: f64, t45406: f64, t45584: f64, t45609: f64, t487: f64, t489: f64, t5463: f64, t5465: f64) -> f64 {
    let t45832 = t42859 * t1243;
    let t45833 = t460 * t45832;
    let t45834 = t43351 * t471;
    let t45838 = -0.23707617058567841754e2_f64 * t12987 * t1280 * t44778 + 0.15805078039045227836e2_f64 * t44843 * t1280 * t44845 - 0.65854491829355115987e0_f64 * t1234 * t1280 * t44878 + 0.39512695097613069592e1_f64 * t3552 * t3787 + 0.52683593463484092788e1_f64 * t5463 * t44639 * t5465 + 0.15805078039045227836e2_f64 * t12966 * t13156 + 0.26341796731742046395e1_f64 * t1285 * t1269 * t12732 * t1287 + 0.65854491829355115987e0_f64 * t460 * t489 * t45406 + 0.65854491829355115987e0_f64 * t1285 * t487 * t45329 * t1287 + 0.26341796731742046395e1_f64 * t13127 * t45584 * t13129 - 0.65854491829355115987e0_f64 * t45833 * t45609 * t45834;
    t45838
}
