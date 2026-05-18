//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1048/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1048<F: Float>(t1009: F, t1704: F, t1003: F, t27772: F, t2811: F, t1008: F, t26686: F, t4796: F, t7718: F, t1020: F, t1121: F, t1804: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27773 = t1009 * t1704;
    let t27774 = t27773 * t1003;
    let t27775 = t27772 * t27774;
    let t27778 = t2811 * t1704;
    let t27779 = t27778 * t1008;
    let t27780 = t26686 * t27779;
    let t27785 = t7718 * t4796;
    let t27786 = t1020 * t27785;
    let t27788 = t1804 * t1121;
    (t27773, t27774, t27775, t27778, t27779, t27780, t27785, t27786, t27788)
}
