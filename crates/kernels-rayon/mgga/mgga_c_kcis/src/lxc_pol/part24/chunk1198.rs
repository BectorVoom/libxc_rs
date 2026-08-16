//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1198/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1198(t7690: f64, t96356: f64, t3329: f64, t8060: f64, t3668: f64, t8104: f64, t15573: f64, t28136: f64, t27077: f64, t26975: f64, t993: f64, t1856: f64, t330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96534 = t7690 * t96356;
    let t96543 = t8060 * t3329;
    let t96670 = t8104 * t3668;
    let t96727 = t15573 * t28136;
    let t96728 = t27077 * t96727;
    let t96735 = t993 * t26975;
    let t96736 = t1856 * t330;
    (t96534, t96543, t96670, t96727, t96728, t96735, t96736)
}
