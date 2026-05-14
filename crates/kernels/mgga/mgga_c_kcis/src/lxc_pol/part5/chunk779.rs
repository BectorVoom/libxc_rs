//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 779/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk779<F: Float>(t1022: F, t6613: F, t1096: F, t1092: F, t1646: F, t1767: F, t3203: F, t3202: F, t3200: F, t1773: F, t3211: F, t3210: F, t6330: F, t1021: F, t1020: F, t1710: F, t4787: F, t4981: F, t5003: F, t5017: F, t5023: F, t6558: F, t6561: F, t6564: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6614 = t1022 * t6613;
    let t6615 = t1096 * t6614;
    let t6616 = t1092 * t6615;
    let t6619 = t1646 * t1767;
    let t6620 = t3203 * t6619;
    let t6621 = t3202 * t6620;
    let t6622 = t3200 * t6621;
    let t6624 = t1646 * t1773;
    let t6625 = t3211 * t6624;
    let t6626 = t3210 * t6625;
    let t6627 = t3200 * t6626;
    let t6629 = t1022 * t6330;
    let t6630 = t1021 * t6629;
    let t6631 = t1020 * t6630;
    let t6633 = -0.13345e0 * t4981 * t1710 - 0.33163888888888888888e-2 * t5017 + 0.22109259259259259258e-2 * t5023 + 0.33163888888888888888e-2 * t4787 + 0.16581944444444444444e-2 * t6558 - 0.49745833333333333332e-2 * t6561 + 0.33163888888888888888e-2 * t6564 - 0.24872916666666666666e-2 * t6616 + 0.22109259259259259258e-2 * t5003 - 0.33163888888888888888e-2 * t6622 + 0.22109259259259259258e-2 * t6627 - 0.33163888888888888888e-2 * t6631;
    (t6614, t6615, t6616, t6620, t6621, t6622, t6625, t6626, t6627, t6629, t6630, t6631, t6633)
}
