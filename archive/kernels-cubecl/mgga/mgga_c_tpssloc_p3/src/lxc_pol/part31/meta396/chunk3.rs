//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1435/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1435<F: Float>(t18583: F, t3578: F, t17691: F, t4972: F, t4582: F, t15615: F, t17686: F, t1155: F, t6069: F, t1695: F, t4857: F, t6088: F) -> (F, F, F, F, F, F) {
    let t18584 = t3578 * t18583;
    let t18589 = t4972 * t17691;
    let t18590 = t4582 * t18589;
    let t18593 = t15615 * t17686;
    let t18594 = t4582 * t18593;
    let t18603 = t6069 * t1155;
    let t18606 = t1695 * t4857;
    let t18609 = t6088 * t1155;
    (t18584, t18590, t18594, t18603, t18606, t18609)
}
