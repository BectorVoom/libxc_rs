//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1396/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1396(t1014: f64, t10864: f64, t10871: f64, t10971: f64, t11177: f64, t11178: f64, t21503: f64, t21507: f64, t21770: f64, t25730: f64, t2578: f64, t2579: f64, t2594: f64, t2602: f64, t2609: f64, t3604: f64, t4310: f64, t4323: f64, t4344: f64, t4345: f64, t6992: f64, t7165: f64, t7222: f64, t9001: f64) -> f64 {
    let t30273 = -0.17315859105681463759e2_f64 * t7222 * t4345 - 0.10254018858216406658e4_f64 * t1014 * t10871 * t21770 - 0.35089341735807877242e1_f64 * t1014 * t10864 * t2579 - 0.10254018858216406658e4_f64 * t1014 * t6992 * t4323 * t9001 - 0.91082604192152556044e5_f64 * t1014 * t21503 * t4310 * t21507 * t2578 + 0.46785788981077169656e1_f64 * t2609 * t10971 - 0.17315859105681463759e2_f64 * t1014 * t10864 * t7165 - 0.35089341735807877242e1_f64 * t1014 * t4344 * t2594 - 0.6233709278045326953e3_f64 * t1014 * t10871 * t2602 + 0.11696447245269292414e1_f64 * t1014 * t11177 * t2594 + 0.23392894490538584828e1_f64 * t2609 * t11178 - 0.34631718211362927518e2_f64 * t1014 * t3604 * t25730;
    t30273
}
