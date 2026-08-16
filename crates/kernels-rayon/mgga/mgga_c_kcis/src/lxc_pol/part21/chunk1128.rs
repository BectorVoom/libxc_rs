//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1128/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1128(t1003: f64, t27773: f64, t27772: f64, t1704: f64, t2811: f64, t1008: f64, t26686: f64, t4796: f64, t7718: f64, t1020: f64, t1121: f64, t1804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27774 = t27773 * t1003;
    let t27775 = t27772 * t27774;
    let t27778 = t2811 * t1704;
    let t27779 = t27778 * t1008;
    let t27780 = t26686 * t27779;
    let t27785 = t7718 * t4796;
    let t27786 = t1020 * t27785;
    let t27788 = t1804 * t1121;
    (t27774, t27775, t27778, t27779, t27780, t27785, t27786, t27788)
}
