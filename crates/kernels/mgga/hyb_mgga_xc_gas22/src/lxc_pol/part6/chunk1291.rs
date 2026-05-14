//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1291/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1291<F: Float>(t1014: F, t10864: F, t10871: F, t10971: F, t11177: F, t11178: F, t21503: F, t21507: F, t21770: F, t25730: F, t2578: F, t2579: F, t2594: F, t2602: F, t2609: F, t3604: F, t4310: F, t4323: F, t4344: F, t4345: F, t6992: F, t7165: F, t7222: F, t9001: F) -> (F,) {
    let t30273 = -0.17315859105681463759e2 * t7222 * t4345 - 0.10254018858216406658e4 * t1014 * t10871 * t21770 - 0.35089341735807877242e1 * t1014 * t10864 * t2579 - 0.10254018858216406658e4 * t1014 * t6992 * t4323 * t9001 - 0.91082604192152556044e5 * t1014 * t21503 * t4310 * t21507 * t2578 + 0.46785788981077169656e1 * t2609 * t10971 - 0.17315859105681463759e2 * t1014 * t10864 * t7165 - 0.35089341735807877242e1 * t1014 * t4344 * t2594 - 0.6233709278045326953e3 * t1014 * t10871 * t2602 + 0.11696447245269292414e1 * t1014 * t11177 * t2594 + 0.23392894490538584828e1 * t2609 * t11178 - 0.34631718211362927518e2 * t1014 * t3604 * t25730;
    (t30273,)
}
