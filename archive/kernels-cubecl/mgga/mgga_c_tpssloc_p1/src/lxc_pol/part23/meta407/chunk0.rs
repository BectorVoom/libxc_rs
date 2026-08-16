//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1220/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1220<F: Float>(t3447: F, t4904: F, t51968: F, t3428: F, t6109: F, t1174: F, t6146: F, t698: F, t6140: F, t11529: F, t6130: F, t15299: F, t4889: F) -> (F, F, F, F, F, F) {
    let t64821 = t3447 * t51968 * t4904;
    let t64878 = t6109 * t3428;
    let t64881 = t1174 * t698 * t6146;
    let t64885 = t1174 * t698 * t6140;
    let t64979 = t1174 * t11529 * t6130;
    let t65002 = t4889 * t15299;
    (t64821, t64878, t64881, t64885, t64979, t65002)
}
