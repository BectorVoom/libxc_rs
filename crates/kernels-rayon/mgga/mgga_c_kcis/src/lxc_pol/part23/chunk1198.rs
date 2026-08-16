//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1198/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1198(t27583: f64, t94934: f64, t27575: f64, t7974: f64, t27651: f64, t7964: f64, t1598: f64, t251: f64, t40541: f64, t27591: f64, t27607: f64, t2257: f64, t2259: f64, t44682: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95130 = t27583 * t94934;
    let t95135 = t27575 * t7974;
    let t95137 = t7964 * t27651;
    let t95143 = t40541 * t251 * t1598;
    let t95157 = t27607 * t27591;
    let t95168 = 0.12871334876543209877e-3_f64 * t2257 * t44682 * t2259;
    (t95130, t95135, t95137, t95143, t95157, t95168)
}
